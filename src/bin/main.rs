<<<<<<< HEAD
use clap::Parser;
=======
use clap::{arg, command, Parser};
extern crate diesel;
>>>>>>> 8033b4b03b9cba47a3fb942b966ad93a950efc7b
use std::fs::{self, metadata, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::{env, io, thread};
mod DirectScan;
mod Zipper;

use diesel::prelude::*;
use judson::{establish_connection, models::*};

#[derive(Parser)]
#[command(author, version)]
struct Args {
    // Add absolute path
    #[arg(short, long)]
    add: Option<String>,

    // View paths and ID
    #[arg(short, long)]
    view: Option<String>,

    // Select zipping type
    #[arg(short, long)]
    format: Option<String>,

    // Target particular absolute path by the ID given to it, {obtain it from view}
    #[arg(short, long)]
    id: Option<String>,
}

fn main() {
    use judson::schema::jud::dsl::*;

    let args = Args::parse();
    println!("Hello {}", args.add.unwrap());

    let connection = &mut establish_connection();

    let results = jud
        .filter(id.is_not_null())
        .limit(5)
        .select(Jud::as_select())
        .load(connection)
        .expect("Error loading posts");
    println!("hi there diesel here");
    println!("Displaying {:?} posts", results);

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

            // Zipper::zipper::zipper(res1, modif, &files, var);
        }
        println! {"\n hello"}
    }
}
