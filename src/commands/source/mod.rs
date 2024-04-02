use clap::Subcommand;
use kanna_commons::{arkalis::Arkalis, repos::source_types::SourceType};

mod add;

#[derive(Debug, Subcommand)]
pub enum SourceCommands {
    Add {
        ///Nome da source
        #[arg(short, long)]
        name: String,
        ///Prioridade
        #[arg(short, long)]
        priority: u32,
        ///Tipo(s)
        #[arg(short, long)]
        types: Vec<SourceType>,
    },
}

pub async fn run(command: SourceCommands, arkalis: Arkalis) -> anyhow::Result<()> {
    match command {
        SourceCommands::Add {
            name,
            priority,
            types,
        } => add::add(name, priority, types, arkalis).await,
    }
}
