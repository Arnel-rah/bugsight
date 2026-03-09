use crate::analyzer;
use crate::config::Config;
use colored::*;
use tiny_http::{Header, Method, Response, Server};

pub fn start(cfg: &Config, port: u16) {
    let addr = format!("0.0.0.0:{}", port);
    let server = match Server::http(&addr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} {}", "Failed to start daemon:".red(), e);
            return;
        }
    };

    println!(
        "\n{} {}",
        "bugsight daemon running on".green().bold(),
        format!("http://localhost:{}", port).cyan().bold()
    );
    println!("{}", "Press Ctrl+C to stop.\n".dimmed());
    println!("{}", "Endpoints:".bold());
    println!(
        "  {} {}",
        "POST".yellow(),
        format!("http://localhost:{}/analyze", port).cyan()
    );
    println!(
        "  {} {}",
        "GET ".yellow(),
        format!("http://localhost:{}/health", port).cyan()
    );
    println!();

    for mut request in server.incoming_requests() {
        let method = request.method().clone();
        let url = request.url().to_string();

        match (method, url.as_str()) {
            (Method::Get, "/health") => {
                let body = serde_json::json!({
                    "status": "ok",
                    "version": env!("CARGO_PKG_VERSION")
                });
                let response = Response::from_string(body.to_string()).with_header(json_header());
                let _ = request.respond(response);
            }

            (Method::Post, "/analyze") => {
                let mut body = String::new();
                let _ = request.as_reader().read_to_string(&mut body);

                let error_input = parse_body(&body);

                let result = match error_input {
                    Some(error) => {
                        println!("{} {}", "→ Analyzing:".yellow(), error.dimmed());

                        match analyzer::analyze(&error, cfg) {
                            Some(result) => {
                                println!("  {} {}", "Type:".bold(), result.error_type.red());
                                println!("  {} {}\n", "Fix:".green().bold(), result.suggestion);

                                serde_json::json!({
                                    "error_type": result.error_type,
                                    "message": result.message,
                                    "suggestion": result.suggestion,
                                    "found": true
                                })
                            }
                            None => {
                                serde_json::json!({
                                    "error_type": null,
                                    "message": error,
                                    "suggestion": null,
                                    "found": false
                                })
                            }
                        }
                    }
                    None => {
                        serde_json::json!({
                            "error": "Missing 'error' field in request body"
                        })
                    }
                };

                let response = Response::from_string(result.to_string()).with_header(json_header());
                let _ = request.respond(response);
            }

            (Method::Options, _) => {
                let response = Response::from_string("").with_header(cors_header());
                let _ = request.respond(response);
            }

            _ => {
                let body = serde_json::json!({
                    "error": "Not found",
                    "endpoints": [
                        "GET  /health",
                        "POST /analyze"
                    ]
                });
                let response = Response::from_string(body.to_string())
                    .with_status_code(404)
                    .with_header(json_header());
                let _ = request.respond(response);
            }
        }
    }
}

fn parse_body(body: &str) -> Option<String> {
    let json: serde_json::Value = serde_json::from_str(body).ok()?;
    json["error"].as_str().map(|s| s.to_string())
}

fn json_header() -> Header {
    Header::from_bytes("Content-Type", "application/json").unwrap()
}

fn cors_header() -> Header {
    Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap()
}
