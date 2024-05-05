use clap::Subcommand;
use kanna_commons::arkalis::Arkalis;

mod add;

#[derive(Debug, Subcommand)]
pub enum EpisodesCommands {
    Add {
        #[arg(short, long)]
        file: String,
        #[arg(short, long)]
        source: u32,
        #[arg(short = 't', long)]
        season: usize,
        #[arg(short = 'n', long)]
        sequence: u32,
        #[arg(short, long)]
        url: String
    },
}

pub async fn run(command: EpisodesCommands, arkalis: Arkalis) -> anyhow::Result<()> {
    match command {
        EpisodesCommands::Add {
            file,
            source,
            season,
            sequence,
            url,
        } => add::add(file, source, season, sequence, url, arkalis).await,
    }
}
