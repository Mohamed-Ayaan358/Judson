use clap::Parser;
use diesel::sql_types::Bool;
use judson::schema::jud;
use std::collections::HashMap;
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
#[derive(Debug)]
struct Args {
    // Add absolute path
    #[arg(short, long, value_delimiter = ',', use_value_delimiter = true)]
    add: Option<Vec<String>>,

    // View paths and ID
    #[arg(short, long)]
    view: Option<bool>,

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
    // println!("{:?}", args);
    let mut addval = match args.add {
        Some(_) => args.add,
        None => None,
    };
    let viewval = match args.view {
        Some(_) => args.view,
        None => None,
    };
    let mut idval = match args.id {
        Some(_) => args.id,
        None => None,
    };
    let mut formatval = match args.format {
        Some(_) => args.format,
        None => None,
    };

    let connection = &mut establish_connection();
    // let mut filesdir: Vec<String> = Vec::new();
    let mut filesdirs: HashMap<String, String> = HashMap::new();

    match addval {
        Some(_) => {
            let id_value = &addval.clone().unwrap()[0];
            let abspath_value = &addval.clone().unwrap()[1];
            let zipmethod_value = &addval.clone().unwrap()[2];

            let new_record = Jud {
                id: id_value.to_string(),
                abspath: abspath_value.to_string(),
                zipmethod: zipmethod_value.to_string(),
            };

            let inserted_rows = diesel::insert_into(jud)
                .values(&new_record)
                .execute(connection);

            let results = jud
                .filter(id.is_not_null())
                .limit(5)
                .select(Jud::as_select())
                .load(connection)
                .expect("Error loading posts");

            for var in results {
                filesdirs.insert(var.abspath, var.zipmethod);
            }
        }
        None => println!(""),
    }

    match viewval {
        Some(_) => {
            let results = jud
                .filter(id.is_not_null())
                .limit(5)
                .select(Jud::as_select())
                .load(connection)
                .expect("Error loading posts");

            for var in results {
                // println!("{:?}", var);
                filesdirs.insert(var.abspath, var.zipmethod);
            }
            // println!("{:?}", filesdirs)
        }

        None => println!("No values present"),
    }

    // let reader = BufReader::new(File::open("files.txt").expect("Cannot open file.txt"));
    // for line in reader.lines() {
    //     filesdir.push(line.unwrap());
    // }
    // println!("\n{:?}", filesdir);

    for (path, method) in filesdirs {
        let res = match DirectScan::directscan::folders(Path::new(&path)) {
            Ok(vecout) => vecout,
            Err(e) => {
                panic!("{}", e);
            }
        };
        println!("{:?}", path);
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
            // DONT REMOVE IT
        }
        println! {"\n "}
    }
}
