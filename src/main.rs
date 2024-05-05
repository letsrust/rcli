// rcli csv -i input.csv -o output.json --header -d ','
// rcli genpass -l 16 --no-lowercase --no-symbol

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use rcli::{
    get_content, get_reader, process_csv, process_decode, process_encode, process_genpass,
    process_text_keygen, process_text_sign, process_text_verify, Base64SubCommand, Opts,
    SubCommand, TextSubCommand,
};
use std::fs;
use zxcvbn::zxcvbn;

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
            let pass = process_genpass(
                opts.length,
                !opts.no_uppercase,
                !opts.no_lowercase,
                !opts.no_number,
                !opts.no_symbol,
            )?;
            println!("{}", pass);

            let estimate = zxcvbn(&pass, &[])?;
            eprintln!("Password strength: {}", estimate.score());
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
        SubCommand::Text(subcmd) => {
            println!("{:?}", subcmd);
            match subcmd {
                TextSubCommand::Sign(opts) => {
                    let mut reader = get_reader(&opts.input)?;
                    let key = get_content(&opts.key)?;
                    let sig = process_text_sign(&mut reader, &key, opts.format)?;
                    let encoded = URL_SAFE_NO_PAD.encode(sig);
                    println!("{}", encoded);
                }
                TextSubCommand::Verify(opts) => {
                    let mut reader = get_reader(&opts.input)?;
                    let key = get_content(&opts.key)?;
                    let decoded = URL_SAFE_NO_PAD.decode(&opts.signature)?;
                    let verified = process_text_verify(&mut reader, &key, &decoded, opts.format)?;
                    if verified {
                        println!("✓ Signature verified");
                    } else {
                        println!("⚠ Signature not verified");
                    }
                }
                TextSubCommand::Generate(opts) => {
                    let key = process_text_keygen(opts.format)?;
                    for (k, v) in key {
                        fs::write(opts.output_path.join(k), v)?;
                    }
                }
            }
        }
    }

    Ok(())
}
