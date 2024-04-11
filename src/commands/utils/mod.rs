use clap::Subcommand;

mod encode;

#[derive(Debug, Subcommand)]
pub enum UtilsCommands {
    /// Re-encoda videos para AV1 usando sua GPU (ou pelo menos tentando)
    ReEncode {
        /// Arquivo ou pasta para ser re-encodado
        #[arg(short, long)]
        path: String,
        /// Arquivo novo
        #[arg(short, long)]
        output: Option<String>,
    },
}

pub async fn run(command: UtilsCommands) -> anyhow::Result<()> {
    match command {
        UtilsCommands::ReEncode { path, output } => encode::encode(path, output).await,
    }
}
