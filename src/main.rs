//use smartcore;
//use automl;

fn print_dataset_names() {
    // print dataset names to user
    println!("1. breast_cancer");
    println!("2. iris");
}

fn set_dataset_name(num: i32) -> Vec<&'static str> {
    // set dataset name based on user input
    // return dataset name
    // unfortunately, these are the only smartcore datasets that work with automl at the moment
    let output = match num {
        1 => "breast_cancer",
        2 => "iris",
        _ => "invalid",
    };
    // return vector of selected dataset name
    vec![output]


}

fn get_user_input() -> i32 {
    // take user input
    // return user input

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
    b1 as i32
}

fn get_dataset() -> String {
    // take user input to get dataset name
    // return dataset_name
    println!("Select dataset from the following options (type number):");
    print_dataset_names();
    let num = get_user_input();
    println!("picked option {}", num);
    let dataset_name = set_dataset_name(num);
    dataset_name[0].to_string()
}

fn main() {
    let name = get_dataset();
    println!("dataset: {:?}", name);
    println!("running automl classifiers...");
    
    let dataset = match name.as_str() {
        "breast_cancer" => smartcore::dataset::breast_cancer::load_dataset(),
        "iris" => smartcore::dataset::iris::load_dataset(),
        // if user input is invalid, return error
        _ => panic!("invalid dataset name"),
    };
    // pass in dataset to automl classifier
    let settings = automl::Settings::default_classification();
    // make classifier variable that uses the dataset and the settings variables above
    let mut classifier = automl::SupervisedModel::new(dataset, settings);
    classifier.train();
    println!("makes it past train");
    print!("{}", classifier);
}
