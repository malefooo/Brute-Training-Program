use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};


// extern crate lettre;
// extern crate lettre_email;

 

fn main() {
//  let email = EmailBuilder::new();
//     // The recipients' addresses for the mail header
//     to("657503492@qq.com");
//     //The sender addresses for the mail header
//     from("Rust_mailSendTest");
//     subject("Hi, LG!");
//     text("This is send mail test!");
//     attachment

    //new email Builder 
    let email = Message::builder()
        .from("Rust <Rust_mailSendTest@domain.tld>".parse().unwrap())
        .to("王海宇 <657503492@qq.com>".parse().unwrap())
        .subject("Hi, LG!")
        .body(String::from("This is send mail test!"))
        .unwrap();
    
    let creds = Credentials::new("1685016883".to_string(), "wanghaiyu,312118".to_string());
   
    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("1685016883@qq.com")
        .unwrap()
        .credentials(creds)
        .build();

    //Send the mail
    match  mailer.send(&eamil) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("email has been failed ,please check with :{:?}", e)
    }
}

