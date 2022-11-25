use actix_files as fs;
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

use graphql_client::reqwest::post_graphql;
use reqwest::Client;

use std::net::TcpListener;

use crate::{queries::*, CaptchaChallenge};

struct AppData {
    cc: CaptchaChallenge,
    tera: Tera,
    phone: String,
    api: String,
}

async fn login(appdata: web::Data<AppData>) -> impl Responder {
    let mut ctx = Context::new();

    ctx.insert("id", &appdata.cc.id);
    ctx.insert("new_captcha", &appdata.cc.new_captcha);
    ctx.insert("failback_mode", &appdata.cc.failback_mode);
    ctx.insert("challenge_code", &appdata.cc.challenge_code);

    let rendered = appdata.tera.render("login.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[derive(Debug, Serialize, Deserialize)]
struct GeetestResponse {
    geetest_challenge: String,
    geetest_seccode: String,
    geetest_validate: String,
}

async fn solve(r: web::Json<GeetestResponse>, appdata: web::Data<AppData>) -> impl Responder {
    println!("Captcha Solved, you may close the browser and return to the CLI.");

    let client = Client::builder().build().expect("Can't build client");

    let input = captcha_request_auth_code::CaptchaRequestAuthCodeInput {
        challenge_code: r.geetest_challenge.clone(),
        phone: appdata.phone.clone(),
        sec_code: r.geetest_seccode.clone(),
        validation_code: r.geetest_validate.clone(),
    };
    let variables = captcha_request_auth_code::Variables { input };

    let response_body =
        post_graphql::<CaptchaRequestAuthCode, _>(&client, appdata.api.clone(), variables).await;

    match response_body {
        Ok(_) => println!("Phone Code sent successfully to {}", appdata.phone),
        Err(e) => {
            log::error!("{:?}", e);
            println!("Phone Code couldn't be send.")
        }
    };

    tokio::spawn(async {
        std::process::exit(0);
    });

    HttpResponse::Ok()
}

pub fn run(
    listener: TcpListener,
    cc: CaptchaChallenge,
    phone: String,
    api: String,
) -> Result<Server, std::io::Error> {
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/public/**/*")).unwrap();

    let appdata = web::Data::new(AppData {
        cc,
        tera,
        phone,
        api,
    });

    let server = HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/static", "src/public/").show_files_listing())
            .route("/login", web::get().to(login))
            .route("/solve", web::post().to(solve))
            .app_data(appdata.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
