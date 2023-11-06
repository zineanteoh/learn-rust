mod run_trait;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use tokio;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    // send back req.body
    HttpResponse::Ok().body(req_body)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    // format!("Hello {}!", &name)
    HttpResponse::Ok().body(format!("Hello {}!", &name))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// this macro enables async/await syntax in the main function
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // run my trait code
    run_trait::run_trait();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
