use rusty_classifiers::{MatLoader, BayesClassifier};

fn main() {
    let train_set = MatLoader::new("data/Data_Train.mat", "Data_Train",
                                  "data/Label_Train.mat", "Label_Train");


    let bayes_classifier = BayesClassifier::new(train_set.get_data(), train_set.get_label(), 0.8);
    let divided = bayes_classifier.group_by_class(&bayes_classifier.get_train_data(), &bayes_classifier.get_train_label());
    println!("{:#?}", divided);
}