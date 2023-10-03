use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;
use std::cmp::Ordering;

#[derive(Debug)]
struct AppState {
    secret_number: u32,
}

#[get("/start_game")]
async fn start_game(data: web::Data<AppState>) -> impl Responder {
    let secret_number = data.secret_number;
    HttpResponse::Ok().json(secret_number)
}

#[derive(Debug)]
struct MakeGuess {
    guess: u32,
}

#[get("/make_guess/{guess}")]
async fn make_guess(data: web::Data<AppState>, web::Path(guess): web::Path<u32>) -> impl Responder {
    let secret_number = data.secret_number;
    match guess.cmp(&secret_number) {
        Ordering::Less => HttpResponse::Ok().body("Too small!"),
        Ordering::Greater => HttpResponse::Ok().body("Too big!"),
        Ordering::Equal => HttpResponse::Ok().body("You win!"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    HttpServer::new(move || {
        App::new()
            .data(AppState { secret_number })
            .service(start_game)
            .service(make_guess)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

