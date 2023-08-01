use ndarray::{self, Slice, Axis, s};
use ndarray::linalg;
use std::collections::HashMap;
use std::hash::Hash;

pub struct BayesClassifier {
    train_data: ndarray::Array2<f64>,
    train_label: ndarray::Array1<f64>,
    val_data: ndarray::Array2<f64>,
    val_label: ndarray::Array1<f64>
}

impl BayesClassifier {
    pub fn new(data: &ndarray::Array2<f64>, label: &ndarray::Array1<f64>, train_test_split: f64) -> Self {
        let train_size = ((data.len_of(Axis(0)) as f64) * train_test_split).floor() as usize;
        Self {
            train_data: data.slice(s![0..train_size, ..]).to_owned(),
            train_label: label.slice(s![0..train_size]).to_owned(),
            val_data: data.slice(s![train_size.., ..]).to_owned(),
            val_label: label.slice(s![train_size..]).to_owned()
        }
    }

    pub fn get_train_data(&self) -> ndarray::Array2<f64> {
        self.train_data.clone()
    }

    pub fn get_train_label(&self) -> ndarray::Array1<f64> {
        self.train_label.clone()
    }

    pub fn train(&self) {
        let trainMap = self.group_by_class(&self.train_data, &self.train_label);
        for (key, value) in &trainMap {
            println!("{}, {}", key, self.calc_mean(&value));
        }
    }

    fn group_by_class(&self, data: &ndarray::Array2<f64>, label: &ndarray::Array1<f64>) -> HashMap<usize, ndarray::Array2<f64>> {
        let mut data_by_class_ndarray: HashMap<usize, ndarray::Array2<f64>> = HashMap::new();
        let mut data_by_class: HashMap<usize, Vec<f64>> = HashMap::new();

        // Determine how many unique classes are there
        for label_i in label {
            let label_u = *label_i as usize;
            data_by_class.entry(label_u).or_insert(Vec::new());
        }

        // Now go through the entire data
        let mut feature_size = data.len();
        let data_length = data.len_of(Axis(0));
        feature_size = feature_size / data_length;
        for i in (0..data_length) {
            let data_slice_vec: Vec<f64> = data.slice(s![i, ..]).iter().cloned().collect();
            let data_key = label[i] as usize;
            data_by_class.get_mut(&data_key).expect("Invalid label!").extend(data_slice_vec);
        }

        for (key, value) in data_by_class {
            let curr_value_length = value.len() / feature_size;
            data_by_class_ndarray.entry(key)
                                 .or_insert(ndarray::Array::from_shape_vec((curr_value_length, feature_size), value)
                                 .expect("Unable to form an array"));
        }
        
        data_by_class_ndarray
    }

    fn calc_mean(&self, data: &ndarray::Array2<f64>) -> ndarray::Array1<f64> {
        data.mean_axis(Axis(0)).expect("Cannot find mean for particular vector!")
    }

    fn calc_var(&self, data: &ndarray::Array2<f64>) -> ndarray::Array1<f64> {
        data.var_axis(Axis(0), 1.)
    }
}