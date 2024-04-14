/*An actix Microservice that has multiple routes:
A.  / that turns a hello world
B. /fruit that returns a random fruit
C. /health that returns a 200 status code
D. /version that returns the version of the service
*/

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use rust_bert::gpt2::GPT2Generator;
use rust_bert::pipelines::generation_utils::{GenerateOptions, LanguageGenerator};

//create a function that returns a hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Lets get started! This is a simple actix web service that has multiple routes: / that acts as a table of contents, /random_question that returns a random question, /health that returns a 200 status code, /version that returns the version of the service.")
}

//create a function that returns a random fruit

//TODO: write a function that outputs a random question to the browser
#[get("/random_question")]
pub async fn random_question() -> impl Responder {
    // Generate a random question
    let model = GPT2Generator::new(Default::default())?;
                                                        
    let input_context_1 = "The dog";
    let input_context_2 = "The cat was";

    let generate_options = GenerateOptions {
        max_length: 30,
        ..Default::default()
    };

    let output = model.generate(Some(&[input_context_1, input_context_2]), generate_options);
    // Return the question as an HTTP response
    HttpResponse::Ok().body(output)
}

//create a function that returns a 200 status code
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}

//create a function that returns the version of the service
#[get("/version")]
async fn version() -> impl Responder {
    //print the version of the service
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    HttpResponse::Ok().body(env!("CARGO_PKG_VERSION"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(random_question)
            .service(health)
            .service(version)
    })
    .bind("0.0.0.0:8090")?
    .run()
    .await
}