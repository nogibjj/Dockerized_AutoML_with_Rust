// use automl;
// use smartcore;
use polars::prelude::*;

// struct dataset_name {
//     name: String,
//     import_line: String,
// }

fn load_custom_dataset() -> Result<DataFrame, PolarsError>  {
    // load in spam.csv into polars df
    let df = CsvReader::from_path("../example_data/iris_clean.csv").unwrap().finish().unwrap();
    println!("{}", df.head(Some(5)));
    Ok(df)
}

/*fn get_dataset() -> dataset_name<String> {
    // take user input to get dataset name
    // return dataset_name
    let mut line = String::new();
    println!("Select dataset from the following options:");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello , {}", line);


}*/

fn main() {
    let dataset = load_custom_dataset();
    // let dataset = smartcore::dataset::boston::load_dataset();
    let settings = automl::Settings::default_classification();
    let mut classifier = automl::SupervisedModel::new(dataset, settings);
    classifier.train();
    print!("{}", classifier);
}


// old code - working as intended for hardcoded dataset
/*fn main() {
    let dataset = smartcore::dataset::boston::load_dataset();
    let settings = automl::Settings::default_classification();
    let mut classifier = automl::SupervisedModel::new(dataset, settings);
    classifier.train();
    print!("{}", classifier);
}*/