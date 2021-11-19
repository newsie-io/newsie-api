use actix_web::{get, App as ActixApp, HttpServer, Responder, web};
use clap::{crate_version, App, Arg};
use serde::Serialize;

// the response to index endpoint requests
#[derive(Serialize)]
struct IndexResponse {
    version: String,
}

// report the version of the API binary we're using, along with
// any other information that would help us debug things
#[get("/")]
pub async fn index() -> impl Responder {
    web::Json(IndexResponse{
        version: crate_version!().to_owned(),
    })
}

// TODO: add other endpoints here

// main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let matches = App::new("Newsie API")
        .version(crate_version!())
        .about("Newsie API server")
        .arg(
            Arg::with_name("LISTEN_ADDR")
                .long("listen-addr")
                .default_value("127.0.0.1:8080"),
        )
        .get_matches();

    // resolve values from the command
    let listen_addr = matches.value_of("LISTEN_ADDR").unwrap();

    // create and start the API server
    HttpServer::new(|| ActixApp::new().service(index))
        .bind(listen_addr)?
        .run()
        .await
}
