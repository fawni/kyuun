use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct KyuunArgs {
    /// The subcommand to run
    #[clap(subcommand)]
    pub command: Option<KyuunCommand>,

    /// The ID of the playlist
    #[arg(short, long)]
    pub id: Option<String>,
}

#[derive(Subcommand)]
pub enum KyuunCommand {
    /// Setup kyuun with the necessary Spotify credentials
    Setup(SetupCommand),
}

#[derive(Args)]
pub struct SetupCommand;
