use crate::client::Arkalis;
use clap::Subcommand;

pub mod login;
pub mod recover;

#[derive(Debug, Subcommand)]
pub enum AuthCommands {
    Login {
        ///Display name
        #[arg(short, long)]
        name: String,
        ///Admin key
        #[arg(short, long)]
        admin_key: Option<String>,
    },
    Recover {
        /// Sua chave de recuperação
        #[arg(short, long)]
        key: String,
    },
}

pub async fn run(command: AuthCommands, client: Arkalis) -> anyhow::Result<()> {
    match command {
        AuthCommands::Login { name, admin_key } => login::login(name, admin_key, client).await,
        AuthCommands::Recover { key } => recover::recover(key, client).await,
    }
}
