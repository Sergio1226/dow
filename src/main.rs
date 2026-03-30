use dow::service;
use clap::Parser;

/// A simple program to Download music from spotify
#[derive(Parser, Debug)]
#[command(author, version, about, about="A simple program to Download music from spotify",long_about = None,arg_required_else_help(true))]
struct Args {
    /// Download a playlist with url or code 
    #[arg(short, long)]
    playlist: Option<String>,

    /// Path to save the downloaded music 
    #[arg(short, long)]
    output: Option<String>,
    
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if let Some(url) = args.playlist {
        let _=service::commands::download_playlist(&url, args.output).await;
    }
    Ok(())
}