mod controllers;
mod models;

mod routes;
mod services;
mod utilities;

#[tokio::main]
async fn main() {
    utilities::server_setup::server().await;
}
