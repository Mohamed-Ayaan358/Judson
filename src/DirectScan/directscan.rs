use itertools::{min, Itertools};
use std::collections::HashSet;
use std::iter::Iterator;
use std::{
    fs::{self, metadata},
    io,
    path::{Path, PathBuf},
};

pub fn folders(dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
    Ok(fs::read_dir(dir)
        .unwrap()
        .into_iter()
        .filter(|r| r.is_ok()) // Get rid of Err variants for Result<DirEntry>
        .map(|r| r.unwrap().path()) // This is safe, since we only have the Ok variants
        .filter(|r| r.is_dir()) // Filter out non-folders
        .collect())
}

pub fn list_files(
    vec: &mut Vec<PathBuf>,
    path: &PathBuf,
    modifying: &mut Vec<u64>,
) -> Result<(Vec<PathBuf>, Vec<u64>), io::Error> {
    let mut value: Vec<PathBuf> = Vec::new();
    // let mut modifying: Vec<u64> = Vec::new();
    if metadata(&path)?.is_dir() {
        let paths = fs::read_dir(&path)?;
        // print!("{}", &path.display());
        for path_result in paths {
            let full_path = path_result?.path();
            if metadata(&full_path)?.is_dir() {
                list_files(vec, &full_path, modifying)?;
            } else {
                // vec.push(full_path);
                let metadata = metadata(&full_path).expect("Failed to get metadata");
                let modification_time = metadata
                    .modified()
                    .expect("Mod")
                    .elapsed()
                    .expect("Mod")
                    .as_secs();
                modifying.push(modification_time / (24 * 60 * 60));
                // if (modification_time / (24 * 60 * 60)) > 15 {
                value.push(path.to_path_buf())
                // }
                // Have to move this out of the loop and depends on smallest value of
            }
        }
    }
    // println!("{:?}", value);s
    // let latest_commit = *modifying.iter().min().unwrap();

    let v: Vec<PathBuf> = value.into_iter().unique().collect();
    let modif: Vec<u64> = modifying.to_vec();
    Ok((v, modif))
}
