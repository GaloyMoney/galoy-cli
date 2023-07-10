use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

use graphql_client::reqwest::post_graphql;
use reqwest::Client;

use std::net::TcpListener;

use crate::client::queries::{captcha_request_auth_code, CaptchaChallenge, CaptchaRequestAuthCode};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

struct AppData {
    tera: Tera,
    phone: String,
    api: String,
    captcha_challenge_result: CaptchaChallenge,
}

async fn login(appdata: web::Data<AppData>) -> impl Responder {
    println!("Fetching Captcha Challenge...");

    let mut ctx = Context::new();

    ctx.insert("id", &appdata.captcha_challenge_result.id);
    ctx.insert("new_captcha", &appdata.captcha_challenge_result.new_captcha);
    ctx.insert(
        "failback_mode",
        &appdata.captcha_challenge_result.failback_mode,
    );
    ctx.insert(
        "challenge_code",
        &appdata.captcha_challenge_result.challenge_code,
    );

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
        channel: None,
    };
    let variables = captcha_request_auth_code::Variables { input };

    let response_body =
        post_graphql::<CaptchaRequestAuthCode, _>(&client, appdata.api.clone(), variables).await;

    match response_body {
        Ok(_) => println!("Phone Code sent successfully to {}", appdata.phone),
        Err(_) => {
            println!("Phone Code couldn't be send.")
        }
    };

    tokio::spawn(async {
        std::process::exit(0);
    });

    HttpResponse::Ok()
}

pub async fn run(
    listener: TcpListener,
    phone: String,
    api: String,
    captcha_challenge_result: CaptchaChallenge,
) -> Result<()> {
    let tera = Tera::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/app/server/public/**/*"
    ))
    .unwrap();

    let appdata = web::Data::new(AppData {
        tera,
        phone,
        api,
        captcha_challenge_result,
    });

    let server = HttpServer::new(move || {
        let generated = generate();
        App::new()
            .service(ResourceFiles::new("/static", generated))
            .route("/login", web::get().to(login))
            .route("/solve", web::post().to(solve))
            .app_data(appdata.clone())
    })
    .listen(listener)?
    .run();

    server.await?;
    Ok(())
}
