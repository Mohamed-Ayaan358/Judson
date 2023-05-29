#![allow(non_snake_case)]

use clap::Parser;
use std::fs::{self, metadata, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::{env, io, thread};
mod DirectScan;
mod Zipper;

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

            Zipper::zipper::zipper(res1, modif, &files, var);
        }
        println! {"\n"}
    }
}
