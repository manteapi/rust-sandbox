use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
fn main() -> Result<()> {
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
    let args = Cli::parse();
    let path = args.path;
    let pattern = args.pattern;
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;
    grrs::matcher::find_matches(&content, &pattern, &mut std::io::stdout())?;
    Ok(())
}
