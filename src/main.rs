// rcli csv -i input.csv -o output.json --header -d ','
// rcli genpass -l 16 --no-lowercase --no-symbol

use clap::Parser;
use rcli::{CmdExecutor, Opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    println!("{:?}", opts);
    opts.cmd.execute().await?;

    Ok(())
}
