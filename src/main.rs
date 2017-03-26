extern crate image;
extern crate img_hash;

// multithreading
extern crate threadpool;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

use std::path::Path;
use std::path::PathBuf;
use std::fs;
use img_hash::{ImageHash, HashType};

struct phash_list_str {
    path : PathBuf,
	phash : img_hash::ImageHash,
}

// takes in a list of files and returns only those with image extensions
fn filter_paths(dir_list: fs::ReadDir) -> Vec<PathBuf> {
    let mut filtered_path_list = Vec::new();
    for entry in dir_list {

        if let Ok(entry) = entry {
            let path = entry.path();
            let file_stem = path.file_stem();
            let file_extension = path.extension();
            let other_path = entry.path();

            match file_extension {
                Some(file_extension) => {
                    let extension = file_extension.to_string_lossy().to_lowercase();
                    //println!("Extension: {}",extension );
                    if extension == "jpg" || extension == "png" || extension == "gif" || extension == "jpeg" {
                        filtered_path_list.push( other_path );
                    }
                }
                None => {
                    println!("Unable to read extension");
                    // ignore path
                }
            }
        }

    }
    filtered_path_list
}

fn calculatePhash (path : &PathBuf) -> img_hash::ImageHash {
    let image_result = image::open(path).unwrap();
    let hash = ImageHash::hash(&image_result, 8, HashType::Gradient);
    return hash
}

fn calculateDifference (first: phash_list_str, second: phash_list_str) {
    println!("% Difference: {}", first.phash.dist_ratio(&second.phash));
}

fn folderWalk(folder_path:&Path)-> Vec<PathBuf> {
    let path_hash_list:&mut Vec<phash_list_str>;

    let raw_paths = fs::read_dir(folder_path).unwrap();
    let image_list = filter_paths(raw_paths);
    return image_list
}

fn main() {

    let image_list = folderWalk(&Path::new("./test_images"));
    let mut path_hash_list:Vec<phash_list_str> = Vec::new();

    println!("Generating pHashes");
    // send off some threads to crunch phashes for these files
    for path in &image_list {
        let hash = calculatePhash(path);
        path_hash_list.push( phash_list_str{path:path.to_path_buf(), phash:hash} );
    }
    for entry in &path_hash_list {
        println!("hash: {:?} path: {:?} size: {:?}", entry.phash.to_base64(), entry.path, entry.phash.size());
    }

    // running difference calculation
    for entry in &path_hash_list {
        for tmpentry in &path_hash_list {
            let difference = entry.phash.dist_ratio(&tmpentry.phash);
            print!("-");
            if difference < 10.0 {
                print!("+");
            }
        }
    }
}
