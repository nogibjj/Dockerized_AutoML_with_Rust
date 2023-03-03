use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// return classifier
fn get_classifier() -> impl Responder {

    let dataset = smartcore::dataset::breast_cancer::load_dataset();
    // pass in dataset to automl classifier
    let settings = automl::Settings::default_classification();
    // make classifier variable that uses the dataset and the settings variables above
    let mut classifier = automl::SupervisedModel::new(dataset, settings);
    classifier.train();
    // print classifier
    HttpResponse::Ok().body(classifier.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //println!("running classifiers first");
    // get classifier first
    //let classifier = get_classifier();
    // cast classifier as string and print
    //println!("{}", classifier);

    // then start server
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            // create a route for the classifier
            //.route("/results", web::get().to(get_classifier))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}