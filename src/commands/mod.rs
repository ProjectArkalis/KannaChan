use std::sync::Arc;

use lazy_static::lazy_static;
use tokio::sync::Mutex;
use tonic::transport::Channel;

use crate::arkalis::arkalis_core_service_client::ArkalisCoreServiceClient;

pub mod login;

// Provavelmente tem algum jeito melhor de fazer isso
lazy_static! {
    static ref CLIENT: Arc<Mutex<Option<ArkalisCoreServiceClient<Channel>>>> = Arc::new(Mutex::new(None));
}

pub async fn set_client(client: ArkalisCoreServiceClient<Channel>) {
    *CLIENT.lock().await = Some(client);
}

async fn get_client() -> ArkalisCoreServiceClient<Channel> {
    CLIENT.lock().await.as_ref().unwrap().clone()
}
