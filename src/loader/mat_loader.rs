use matfile::{self, NumericData};
use ndarray;

pub struct MatLoader {
    // data - actual observation value
    // label - corresponding observation value label (index is synchronous)
    data: ndarray::Array2<f64>,
    label: ndarray::Array1<f64>,
}

impl MatLoader {
    pub fn new(data_file: &str, data_array: &str, label_file: &str, label_array: &str) -> Self {
        let packed_data: matfile::Array = load(data_file, data_array);
        let packed_label: matfile::Array = load(label_file, label_array);
        let data_shape = packed_data.size();

        let vec_data = match packed_data.data() {
            NumericData::Double { real, imag: _ } => real.clone(),
            _ => panic!("Invalid observations loaded!")
        };
        
        let vec_label = match packed_label.data() {
            NumericData::Double { real, imag: _ } => real.clone(),
            _ => panic!("Invalid observations loaded!")
        };

        // Convert data and label into individual ndarray::Array types
        let data: ndarray::Array2<f64> = ndarray::Array::from_shape_vec((data_shape[0], data_shape[1]), vec_data).unwrap();
        let label: ndarray::Array1<f64> = ndarray::Array::from_shape_vec(data_shape[0], vec_label).unwrap();

        return Self {
            data,
            label
        }
    }

    pub fn get_data(&self) -> &ndarray::Array2<f64> {
        &self.data
    }

    pub fn get_label(&self) -> &ndarray::Array1<f64> {
        &self.label
    }

}

pub fn load(file_location: &str, array_name: &str) -> matfile::Array {
    let file = std::fs::File::open(file_location).expect("Cannot open file");
    let parsed_file = matfile::MatFile::parse(file).expect("Error loading file!");
    let curr_data  = parsed_file.find_by_name(array_name).expect("Can't find stored array");
    curr_data.clone()
}