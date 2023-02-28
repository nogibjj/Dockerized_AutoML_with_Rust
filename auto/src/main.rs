fn print_dataset_names() {
    println!("1. boston");
    println!("2. breast_cancer");
    println!("3. diabetes");
    println!("4. digits");
    println!("5. iris");
}

fn set_dataset_name(num: i32) -> Vec<&'static str> {
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
// create get user input function that returns an i32
fn get_user_input() -> i32 {
    // create variable named line that takes user input (string)
    let mut line = String::new();
    // create variable named b1 that takes user input (int)
    let b1 = loop {
        // take user input
        std::io::stdin().read_line(&mut line).unwrap();
        // convert user input to int
        let b1: usize = line.trim().parse().unwrap();
        // check if user input is valid
        if b1 > 0 && b1 < 6 {
            break b1;
        } else {
            println!("Invalid input. Please try again.");
            line.clear();
        }
    };
    println!("b1: {}", b1);
    b1 as i32
}

fn get_dataset() -> String {
    // take user input to get dataset name
    // return dataset_name
    let line = String::new();
    println!("Select dataset from the following options (type number):");
    print_dataset_names();
    let num = get_user_input();
    let dataset_name = set_dataset_name(num);
    println!("picked option {}", line);
    // return dataset_name
    dataset_name[0].to_string()
}

fn main() {
    //let dataset = load_custom_dataset();
    let name = get_dataset();
    println!("dataset: {:?}", name);
    println!("running automl classifiers...")
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
