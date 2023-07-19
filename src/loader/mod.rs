use matfile;

struct Data {
    // data - actual observation value
    // label - corresponding observation value label (index is synchronous)
    data: Vec<f32>,
    label: Vec<i32>
}

impl Data {
    pub fn new(data_file: &str, label_file: &str) -> Self {
        let option_data: matfile::MatFile = load(data_file).expect("Cannot load data file!");
        let option_label: matfile::MatFile = load(label_file).expect("Cannot load label file!");
        
        // Extract the data
        if let Some(curr_data) = option_data.find_by_name("real") {
            println!("Extracted data {:#?}", curr_data);

            // Extract the label
            if let Some(curr_label) = option_label.find_by_name("real") {
                println!("Extracted data {:#?}", curr_label);
         
                return Self {
                    data: curr_data,
                    label: curr_label
                }
            }
        } else {
            panic!("Cannot extract data!");
            Self {

            }
        }
    }
}

fn load(file_location: &str) {
    let file = std::fs::File::open(file_location).expect("Cannot load file {}", file_location);
    let parsed_file_result = matfile::MatFile::parse(file);
}