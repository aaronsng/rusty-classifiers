use matfile;
use std::fmt::Error;

struct Data {
    // data - actual observation value
    // label - corresponding observation value label (index is synchronous)
    data: Vec<f32>,
    label: Vec<i32>
}

impl Data {
    pub fn new(data_file: &str, label_file: &str) -> () {
        let option_data = load(data_file).expect("Cannot load data file!");
        let option_label = load(label_file).expect("Cannot load label file!");
    }
}

pub fn load(file_location: &str) -> Result<Vec<T>, Error> {
    let file = std::fs::File::open(file_location).expect("Cannot load file");
    let parsed_file_result = matfile::MatFile::parse(file);
    let parsed_file = match parsed_file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("Error loading file! {:?}", error);
        }
    };

    println!("{:#?}", parsed_file);

    let curr_data = parsed_file.find_by_name("array").expect("Can't find stored array");
    
}