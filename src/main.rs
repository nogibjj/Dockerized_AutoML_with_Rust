fn main() {

    let dataset = smartcore::dataset::breast_cancer::load_dataset();
    // pass in dataset to automl classifier
    let settings = automl::Settings::default_classification();
    // make classifier variable that uses the dataset and the settings variables above
    let mut classifier = automl::SupervisedModel::new(dataset, settings);
    classifier.train();
    println!("makes it past train");
    print!("{}", classifier);
}