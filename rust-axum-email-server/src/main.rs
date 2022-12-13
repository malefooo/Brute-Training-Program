use axum::{
    response::{Html,IntoResponse},
    routing::{get,post},
    Json, Router
};
use dotenv::dotenv;//import .env file
use serde::{Deserialize,Serialize};
use std::env;
use std::net::SocketAddr;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message,SmtpTransport,Transport};

#[derive(Debug,Serialize,Deserialize)]
struct EmailPayload {
    fullname: String,
    email: String,
    message: String,
}

//make "async" keyword take effect 
#[tokio::main]
async fn main() {
    dotenv().ok();
    
    //crete a application with a route
    let app = Router::new()
        .route("/",get(handler))
        .route("/send-email", post(dispatch_email));

    //run this application
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("starting on http://{}", addr);
    //Axum example
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

//here can handld http request and resonse
async fn handler() -> Html<String> {
    Html("<h1>Hello, World!</h1>".to_string())
}

//here is send email fuction
async fn dispatch_email(Json(payload): Json<EmailPayload>) -> impl IntoResponse {
    let EmailPayload {
        email,
        message,
        fullname,
    } = &payload;

    let from_address = String::from("LG <1685016883@qq.com>");
    let to_address = format!("{fullname} <{email}>");
    let reply_to = String::from("LG <1685016883@qq.com>");
    let email_subject = "Send Email Server test";
    
    let email = Message::builder()
    .from(from_address.parse().unwrap())
    .reply_to(reply_to.parse().unwrap())
    .to(to_address.parse().unwrap())
    .subject(email_subject)
    .body(String::from(message)).unwrap();

    //save username & password
    let creds = Credentials::new(
        env::var("SMTP_USERNAME").expect("SMTP Username not specified"),
        env::var("SMTP_PASSWORD").expect("SMTP Password not specified")
    );

    //Open remote connection to SMTP server
    let mailer = SmtpTransport::relay(&env::var("SMTP_HOST").expect("SMTP Host not specified"))
    .unwrap()
    .credentials(creds)
    .build();

    //send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("there has some issue in your server: {:?}", e),
    }

}

