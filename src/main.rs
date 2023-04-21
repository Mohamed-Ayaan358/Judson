#![allow(non_snake_case)]

use itertools::min;
use std::fs::{self, metadata};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{io, thread};
use zip_archive::Archiver;
mod DirectScan;

// Maybe make value into a dictionary (text:modification time)
fn main() {
    // let result = DirectScan::directscan::folders(Path::new("/home/ayaan/Desktop/Coding")).unwrap();
    let res = match DirectScan::directscan::folders(Path::new("/home/ayaan/Desktop/Coding")) {
        Ok(vecout) => vecout,
        Err(e) => {
            panic!("{}", e);
        }
    };
    let mut valuedir: Vec<PathBuf> = Vec::new();

    let mut value: Vec<u64> = Vec::new();
    for var in res {
        let mut metadata = metadata(var.clone()).expect("Failed to get metadata");
        let mut modification_time = metadata
            .modified()
            .expect("Mod")
            .elapsed()
            .expect("Mod")
            .as_secs();
        value.push(modification_time);

        let mut vec = Vec::new();
        let mut modifying: Vec<u64> = Vec::new();
        let (res1, modif) = match DirectScan::directscan::list_files(&mut vec, &var, &mut modifying)
        {
            Ok(vecout) => vecout,
            Err(e) => {
                panic!("{}", e);
            }
        };

        if !res1.is_empty() {
            let latest_commit = *modif.iter().min().unwrap();
            let dir_names = res1.iter().nth(0).unwrap();
            valuedir.push(dir_names.to_path_buf());
            // if latest_commit > 10 {
            //     println!("{:?} {:?}", res1.iter().nth(0).unwrap(), latest_commit);
            // }
            println!("{:?} {:?}", res1.iter().nth(0).unwrap(), min(modif));
        }
        // println!("{:?} {:?}", res1.iter().nth(0).unwrap(), min(modif));
    }
    // println!("{:?}", valuedir[4]);

    // for dir in valuedir {
    // let origin = PathBuf::from(&dir);
    // let dest = PathBuf::from("./dest");
    // // let thread_count = 16;

    // let mut archiver = Archiver::new();
    // archiver.push(origin);
    // archiver.set_destination(dest);
    // // archiver.set_thread_count(thread_count);

    // match archiver.archive() {
    //     Ok(_) => (println!("Built!")),
    //     Err(e) => println!("Cannot archive the directory! {}", e),
    // }
    // // }
}
