use itertools::min;
use std::fs::{self, metadata, File};
use std::path::{Path, PathBuf};
use zip_archive::Archiver;

pub fn zipper(res1: Vec<PathBuf>, modif: Vec<u64>, files: &String, var: PathBuf) {
    if !res1.is_empty() {
        let latest_commit = *modif.iter().min().unwrap();

        // if nodemodrm == "true" {} If we are gonna use args

        let javascript: String =
            res1.iter().nth(0).unwrap().as_path().display().to_string() + "/node_modules";
        fs::remove_dir_all(javascript).expect("No node_modules found");

        if latest_commit > 15 {
            // valuedir.push(dir_names.to_path_buf());
            let origin = PathBuf::from(res1.iter().nth(0).unwrap());
            let dest = PathBuf::from(&files);
            let thread_count = 8;

            let mut archiver = Archiver::new();
            archiver.push(origin);
            archiver.set_destination(dest);
            archiver.set_thread_count(thread_count);

            match archiver.archive() {
                Ok(_) => (println!("Built!")),
                Err(e) => println!("Cannot archive the directory! {}", e),
            }
            fs::remove_dir_all(var).unwrap();
        }
        println!("{:?} {:?}", res1.iter().nth(0).unwrap(), min(modif));
    }
}
