// use smartcore;
use polars::prelude::*;

/*struct DatasetName {
    name: String,
    import_line: String,
}*/

fn print_dataset_names() {
    println!("1. boston");
    println!("2. breast_cancer");
    println!("3. diabetes");
    println!("4. digits");
    println!("5. iris");
}

// create set_dataset_name function which takes an i32 number as parameter
// and returns a vector of strings

fn set_dataset_name(num: usize) -> Vec<&'static str> {
    // set dataset name based on user input
    // return dataset_name
    let output = match num {
        1 => "boston",
        2 => "breast_cancer",
        3 => "diabetes",
        4 => "digits",
        5 => "iris",
        _ => "invalid",
    };
    // return vector of selected dataset name
    vec![output]


}

fn get_dataset() -> String {
    // take user input to get dataset name
    // return dataset_name
    let mut line = String::new();
    println!("Select dataset from the following options (type number):");
    print_dataset_names();

    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    let dataset_name = set_dataset_name(b1);
    println!("picked option {}", line);
    // return dataset_name
    dataset_name[0].to_string()
}

/*fn load_custom_dataset() -> Result<DataFrame, PolarsError> {
    // load in spam.csv into polars df
    let df = CsvReader::from_path("../example_data/iris_clean.csv")
        .unwrap()
        .finish()
        .unwrap();
    println!("{}", df.head(Some(5)));
    Ok(df)
}*/

fn main() {
    //let dataset = load_custom_dataset();
    let name = get_dataset();
    println!("dataset: {:?}", name);
    /*
    format + lint after copilot suggestion
    */
    //let dataset = smartcore::dataset::boston::load_dataset();

    // take user input from name variable to load into smartcore dataset
    let dataset = match name.as_str() {
        "boston" => smartcore::dataset::boston::load_dataset(),
        "breast_cancer" => smartcore::dataset::breast_cancer::load_dataset(),
        "diabetes" => smartcore::dataset::diabetes::load_dataset(),
        "digits" => smartcore::dataset::digits::load_dataset(),
        "iris" => smartcore::dataset::iris::load_dataset(),
        _ => smartcore::dataset::iris::load_dataset(),
    };

    // pass in dataset to automl classifier

    let settings = automl::Settings::default_classification();
    // make classifier variable that uses the dataset and the settings variables above
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
