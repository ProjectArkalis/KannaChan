#![feature(type_alias_impl_trait)]

use std::sync::Arc;
use crate::configs::Configs;
use arguments::{Cli, Commands};
use arkalis::arkalis_core_service_client::ArkalisCoreServiceClient;
use clap::Parser;
use commands::login;
use tonic::{service::{interceptor::InterceptedService, Interceptor}, transport::Channel, Request, Status};

mod arguments;
mod configs;
mod models;
mod commands;

//isso vai dar merda
type Arkalis = ArkalisCoreServiceClient<InterceptedService<Channel, AuthInterceptor>>;

pub mod arkalis {
    tonic::include_proto!("arkalis");
}

lazy_static::lazy_static! {
    pub static ref CONFIGS: Configs = Configs::get();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let channel = Channel::from_static(&CONFIGS.arkalis_url).connect().await?;

    let mut client = ArkalisCoreServiceClient::with_interceptor(channel, AuthInterceptor);
    

    // set_client(a).await;

    match args.command {
        Commands::Login { name, admin_key } => login::login(name, admin_key, client).await,
    }
}


pub struct AuthInterceptor;

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        if let Some(token) = &CONFIGS.token {
            request.metadata_mut().insert("", format!("Bearer {}", token).parse().unwrap());
        }
        Ok(request)
    }
}