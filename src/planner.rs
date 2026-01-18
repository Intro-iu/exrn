use crate::file_utils::normalize_path;
use glob::glob;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenameAction {
    pub source: PathBuf,
    pub target: PathBuf,
}

pub fn find_matches(
    patterns: &[String],
    source_regex: &Regex,
) -> Result<HashSet<PathBuf>, Box<dyn Error>> {
    let mut matched_files = HashSet::new();
    for pattern in patterns {
        // 1. Check if it's a specific existing file (handles unquoted args with special chars correctly)
        // Shell expansion typically provides reachable paths.
        let path = Path::new(pattern);
        if path.is_file() {
            let abs_path = normalize_path(pattern)?;
            let file_name = abs_path
                .file_name()
                .ok_or("Invalid file name")?
                .to_string_lossy();
            if source_regex.is_match(&file_name) {
                matched_files.insert(abs_path);
            }
            continue;
        }

        // 2. Treat as glob pattern (quoted wildcards or non-existent paths)
        for entry in glob(pattern)? {
            let path = normalize_path(&entry?.to_string_lossy())?;
            if path.is_file() {
                let file_name = path
                    .file_name()
                    .ok_or("Invalid file name")?
                    .to_string_lossy();
                if source_regex.is_match(&file_name) {
                    matched_files.insert(path);
                }
            }
        }
    }
    Ok(matched_files)
}

pub fn generate_new_path(
    file_path: &Path,
    source_regex: &Regex,
    target_pattern: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let file_name = file_path
        .file_name()
        .ok_or("Invalid file name")?
        .to_string_lossy();

    let new_name = source_regex.replace_all(&file_name, target_pattern);

    if new_name.is_empty() {
        return Err("New file name cannot be empty".into());
    }
    if new_name.contains(std::path::MAIN_SEPARATOR) {
        return Err(format!("New name contains path separator: {}", new_name).into());
    }

    Ok(file_path.with_file_name(new_name.as_ref()))
}

// Generate execution plan with topological sort
pub fn generate_plan(
    matched_files: HashSet<PathBuf>,
    source_regex: &Regex,
    target_pattern: &str,
) -> Result<Vec<RenameAction>, Box<dyn Error>> {
    let mut actions = Vec::new();
    let mut source_set = HashSet::new();
    let mut target_to_source = HashMap::new();

    // 1. Generate initial targets
    for source in matched_files {
        let target = generate_new_path(&source, source_regex, target_pattern)?;
        if source != target {
            if target_to_source.contains_key(&target) {
                return Err(format!("Collision detected: Multiple files map to {}", target.display()).into());
            }
            target_to_source.insert(target.clone(), source.clone());
            source_set.insert(source.clone());
            actions.push(RenameAction { source, target });
        }
    }

    if actions.is_empty() {
        return Ok(Vec::new());
    }

    // 2. Validate Targets
    for action in &actions {
        if action.target.exists() && !source_set.contains(&action.target) {
            return Err(format!(
                "Target file already exists and is not being renamed: {}",
                action.target.display()
            )
            .into());
        }
    }

    // 3. Topological Sort for Execution Order
    // Graph:
    // Nodes: Indices in `actions`
    // Edge: i -> j means "Action i must happen AFTER Action j"
    // condition: Action i (A->B) requires B to be free.
    // If B is being moved by Action j (B->C), then i depends on j. (i waits for j)
    
    let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut in_degree: HashMap<usize, usize> = HashMap::new();
    
    let path_to_action_idx: HashMap<PathBuf, usize> = actions
        .iter()
        .enumerate()
        .map(|(i, a)| (a.source.clone(), i))
        .collect();

    for i in 0..actions.len() {
        in_degree.entry(i).or_insert(0);
        let target = &actions[i].target;
        // If target is someone else's source, that someone else (j) must run first.
        // So i depends on j. (Wait for j to clear the spot)
        if let Some(&j) = path_to_action_idx.get(target) {
             // Edge: i -> j (i depends on j)
             // Wait, standard topo sort moves from 0 in-degree.
             // If i depends on j, we want j to be executed first.
             // So in the output list, j comes before i.
             // Standard topo sort: build graph where edge u->v means u comes before v.
             // Here: j comes before i. So edge j -> i.
             
             // Let's re-verify:
             // i: A->B
             // j: B->C
             // We need j then i. (B moves to C, freeing B for A).
             // So j -> i.
             
             adj.entry(j).or_default().push(i);
             *in_degree.entry(i).or_insert(0) += 1;
        }
    }

    // Kahn's Algorithm
    let mut queue: Vec<usize> = in_degree
        .iter()
        .filter(|&(_, &d)| d == 0)
        .map(|(&i, _)| i)
        .collect();
    
    // Sort queue to make deterministic if needed, though Set/Map iteration order varies
    // But logic correctness doesn't depend on it.

    let mut sorted_indices = Vec::new();
    while let Some(u) = queue.pop() {
        sorted_indices.push(u);
        if let Some(neighbors) = adj.get(&u) {
            for &v in neighbors {
                let d = in_degree.get_mut(&v).unwrap();
                *d -= 1;
                if *d == 0 {
                    queue.push(v);
                }
            }
        }
    }

    if sorted_indices.len() != actions.len() {
        return Err("Circular dependency detected (e.g. A->B, B->A). Cannot safely rename without temporary files.".into());
    }

    let sorted_actions = sorted_indices.into_iter().map(|i| actions[i].clone()).collect();
    Ok(sorted_actions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn test_chain_renaming_prepend() {
        let dir = TempDir::new().unwrap();
        let path_a = dir.path().join("a");
        let path_xa = dir.path().join("xa");
        
        File::create(&path_a).unwrap();
        File::create(&path_xa).unwrap();
        
        let mut matched = HashSet::new();
        matched.insert(path_a.clone());
        matched.insert(path_xa.clone());
        
        let regex = Regex::new("^(.*)$").unwrap();
        // Remove underscore to ensure "a"->"xa" matches existing "xa"
        let target = "x$1";
        
        let plan = generate_plan(matched, &regex, target).unwrap();
        
        assert_eq!(plan.len(), 2);
        // Correct order: xa -> xxa, then a -> xa
        // Verify output names
        let first = &plan[0]; 
        let second = &plan[1];
        
        // xa -> xxa
        if first.source == path_xa {
            assert_eq!(first.target, dir.path().join("xxa"));
            // Second should be a -> xa
            assert_eq!(second.source, path_a);
            assert_eq!(second.target, path_xa); 
        } else {
            // Check if order is wrong
            panic!("Wrong order! expected xa to move first: {:?} -> {:?}", first.source, first.target);
        }
    }
}
