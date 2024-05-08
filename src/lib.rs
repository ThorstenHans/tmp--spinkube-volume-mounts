use std::fs;

use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_repro(_req: Request) -> anyhow::Result<impl IntoResponse> {
    let content = fs::read_to_string("/mount/.gitkeep")?;
    println!("Read /mount/.gitkeep: '{}'", content);
    let content = fs::read_to_string("/mount/secrets/some")?;
    println!("Read /mount/secrets/some: '{}'", content);
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, Fermyon")
        .build())
}
