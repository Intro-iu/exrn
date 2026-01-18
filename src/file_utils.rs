use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

// Normalize path to absolute path
pub fn normalize_path(path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let path = Path::new(path);
    Ok(if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    })
}

// Execute rename operation
pub fn safe_rename(source: &Path, target: &Path) -> Result<(), Box<dyn Error>> {
    if source == target {
        return Err("Source and target are the same".into());
    }
    if target.exists() {
        // This check might be redundant if planner guarantees safety, 
        // but good as a final safety net.
        // However, for chain renaming A->B where B is also moving B->C, 
        // B WILL exist when we rename A->B if we don't order them correctly.
        // But the planner should strictly order them so B is gone before A moves there.
        // So checking target.exists() here is actually tricky for chain renaming IF we strictly rely on ordering.
        // IF we rely on ordering, B is gone.
        // BUT, what if B was a file that was NOT part of the rename set? Then we must error.
        // So we should check if target exists, UNLESS we know it's being moved away.
        // But functions here should be stateless. 
        // Let's keep strict safety here: if target exists on disk, fail.
        // The planner must ensure target does NOT exist (i.e. has been moved) before calling this.
        return Err("Target file already exists".into());
    }
    fs::rename(source, target)?;
    Ok(())
}
