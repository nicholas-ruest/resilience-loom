use clap::Parser;
#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    version: bool,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = Args::parse();
    if a.version {
        println!("resilience-loom 0.1.0");
        return Ok(());
    }
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .json()
        .init();
    rl_api::serve(rl_config::Config::from_env()?).await?;
    Ok(())
}
