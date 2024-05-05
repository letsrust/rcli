// rcli csv -i input.csv -o output.json --header -d ','
// rcli genpass -l 16 --no-lowercase --no-symbol

use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Opts,
    SubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
            // println!("{}", opts.output.is_none())
        }
        SubCommand::GenPass(opts) => {
            println!("{:?}", opts);
            process_genpass(
                opts.length,
                !opts.no_uppercase,
                !opts.no_lowercase,
                !opts.no_number,
                !opts.no_symbol,
            )?;
        }
        SubCommand::Base64(subcmd) => {
            println!("{:?}", subcmd);
            match subcmd {
                Base64SubCommand::Encode(opts) => {
                    process_encode(&opts.input, opts.format)?;
                }
                Base64SubCommand::Decode(opts) => {
                    process_decode(&opts.input, opts.format)?;
                }
            }
        }
    }

    Ok(())
}
