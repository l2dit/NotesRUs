use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Port For The Server
    #[arg(short, long, env, default_value_t = 3000)]
    pub port: u16,

    /// What Request Origns Are Valid
    #[arg(short, long, env, default_value_t = String::from("0.0.0.0"))]
    pub origns: String,

    /// Is The Aplication Running In Kubernetes
    #[arg(short, long, env, default_value_t = String::from("localhost"))]
    pub domain: String,

    /// Weather Your Server Is Being Reached From Https:// Assumes Port (443)
    #[arg(long, env, default_value_t = false)]
    pub https: bool,

    /// Postgresql Username
    #[arg(long, env)]
    pub database_username: Option<String>,

    /// Postgresql Password
    #[arg(long, env)]
    pub database_password: Option<String>,

    /// Postgresql Connection IP
    #[arg(long, env)]
    pub database_ip: Option<String>,

    /// Postgresql Connection Port
    #[arg(long, env)]
    pub database_port: Option<u16>,
}

/// Create The Server String
pub fn server_constructor(
    domain: &String,
    port: u16,
    suffix: Option<String>,
    https: Option<bool>,
) -> String {
    match https {
        Some(true) => return format!("https://{domain}{}", suffix.unwrap_or(String::new())),

        Some(false) => return format!("http://{domain}:{port}{}", suffix.unwrap_or(String::new())),

        None => return format!("{domain}:{port}"),
    }
}
