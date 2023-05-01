#![allow(non_snake_case)]

use itertools::min;
use std::fs::{self, metadata, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{env, io, thread};
use zip_archive::Archiver;
mod DirectScan;

// Maybe make value into a dictionary (text:modification time)
fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // let nodemodrm = &args[1]; If we are gonna use args

    let mut filesdir: Vec<String> = Vec::new();
    let reader = BufReader::new(File::open("files.txt").expect("Cannot open file.txt"));

    for line in reader.lines() {
        filesdir.push(line.unwrap());
    }
    println!("{:?}", filesdir);
    for files in filesdir {
        let res = match DirectScan::directscan::folders(Path::new(&files)) {
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
            let (res1, modif) =
                match DirectScan::directscan::list_files(&mut vec, &var, &mut modifying) {
                    Ok(vecout) => vecout,
                    Err(e) => {
                        panic!("{}", e);
                    }
                };

            if !res1.is_empty() {
                let latest_commit = *modif.iter().min().unwrap();

                // if nodemodrm == "true" {} If we are gonna use args
                let javascript: String =
                    res1.iter().nth(0).unwrap().as_path().display().to_string() + "/node_modules";
                fs::remove_dir_all(javascript).unwrap();

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
                // println!("{:?} {:?}", res1.iter().nth(0).unwrap(), min(modif));
            }
        }
        println! {"\n"}
    }
}
