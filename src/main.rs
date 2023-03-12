use std::{fs::File, io::{Read, Write}};

use rand::{seq::SliceRandom, thread_rng};
use regex::Regex;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let script_name = &args.get(1).unwrap_or_else(|| {
        panic!("Cannot find first argument (original script)");
    }); 
    let output_name = &args.get(2).unwrap_or_else(|| {
        panic!("Cannot find second argument (output script name)");
    });

    // Open and read file
    let mut file = File::open(script_name).unwrap_or_else(|err| {
        panic!("Failed to open the file with name {}: {}", script_name, err);
    });
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap_or_else(|err| {
        panic!("Failed to read file: {}", err);
    });

    // Randomize words
    let regex = Regex::new(r"[^ \n]+").unwrap();
    let mut words = Vec::new();
    for mat in regex.find_iter(&contents) {
        let mat = mat.as_str();
        if !mat.is_empty() {
            words.push(mat.trim());
        }
    };
    words.shuffle(&mut thread_rng());

    // Create and write file
    let mut out_file = File::create(output_name).unwrap_or_else(|err| {
        panic!("Failed to make a file with the name {}: {}", output_name, err)
    });
    out_file.write_all(words.join(" ").as_bytes()).unwrap_or_else(|err| {
        panic!("Failed to write file with the script: {}", err);
    });
    println!("Finished! File saved to {}", output_name);

}
