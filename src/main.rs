use serde_derive::Deserialize;
use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use std::process::Command;

#[derive(Deserialize)]
struct ScriptRequest {
    script: String,
}

#[post("/execute")]
async fn execute_script(req_body: web::Json<ScriptRequest>) -> impl Responder {
    let output = Command::new("sh")
        .arg("-c")
        .arg(&req_body.script)
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            HttpResponse::Ok().json(serde_json::json!({
                "stdout": stdout,
                "stderr": stderr,
                "status": output.status.code()
            }))
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to execute script: {}", e)),
    }
}

#[get("/")]
async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Shell Executor Service!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(welcome)
            .service(execute_script)
    })
        .bind("0.0.0.0:8888")?
        .run()
        .await
}
