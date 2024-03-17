use crate::{arkalis::arkalis_core_service_client::ArkalisCoreServiceClient, CONFIGS};
use tonic::{
    service::{interceptor::InterceptedService, Interceptor},
    transport::Channel,
    Request, Status,
};

pub type Arkalis = ArkalisCoreServiceClient<InterceptedService<Channel, AuthInterceptor>>;

pub struct AuthInterceptor {
    token: Option<String>,
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        if let Some(token) = &self.token {
            request
                .metadata_mut()
                .insert("authorization", format!("Bearer {}", token).parse().unwrap());
        }
        Ok(request)
    }
}

pub async fn get_client(token: Option<String>) -> anyhow::Result<Arkalis> {
    let channel = Channel::from_static(&CONFIGS.arkalis_url).connect().await?;
    let client = ArkalisCoreServiceClient::with_interceptor(channel, AuthInterceptor { token });
    Ok(client)
}
