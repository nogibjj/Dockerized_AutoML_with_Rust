# AutoML_Kubernetes
Utilizing Rust to build an AutoML application. Will be deployed to a Kubernetes service

## plan going forward
- take boiler plate code from automl crate - automate it as much as i can
    - choose from a variety of datasets? input your own data?
    - pick dataset from smartcore options in cmd line tool format?
- get all models back, make a function to choose the best one
    - host model on a cloud service, then predict?
    - serve predictions to some other cloud env?

## end goal
command line tool that you can pass any (cleaned) dataset into (already ohe'd, correct format, target specified, etc) and automl (only classifier for now) runs all models on it. somehow i'd like to get this deployed as a kubernetes service