use std::fs::File;
use std::io::prelude::*;
use serde_json;
use serde_json::Value;

struct FileLocations {
    files : Vec<Group>
}

struct Group {
    subject: String,
    location: String
} 

pub fn get_link(subject: String, find_url: String) -> String {
    let content: String = get_content_from_file(subject);
    let content: Value = serde_json::from_str(&content).unwrap();
    content[find_url].as_str().unwrap().to_string()
}

pub fn get_content_from_file(subject: String) -> String {
    let location : String = get_location(subject); 
    let mut f = File::open(location).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

fn get_location(subject: String) -> String {
    let file_groups : FileLocations = get_files();
    let mut correct_location : String = String::new();

    let total_groups: usize = file_groups.files.len();
    let mut current_position: usize = 0;
    while current_position < total_groups {
        if &file_groups.files[current_position].subject == &subject {
            correct_location =  file_groups.files[current_position].location.clone();
        };
        current_position = current_position + 1;
    }

    correct_location
}

fn get_files()  -> FileLocations {
    let group_one = Group {
        subject : String::from("gulden_request"),
        location : String::from("gulden_requests.json"),
    };

    let new_groups = FileLocations {
        files : vec![group_one],
    };

    new_groups
}