use crate::{process_jwt_sign, process_jwt_verify, CmdExecutor};
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum JwtSubCommand {
    #[command(name = "sign", about = "Sign with a key and return a JWT token")]
    Sign(JwtSignOpts),
    #[command(name = "verify", about = "Verify a JWT token with a key")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(long)]
    pub sub: String,
    #[arg(long)]
    pub aud: String,
    #[arg(long)]
    pub exp: String,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long)]
    pub token: String,
}

impl CmdExecutor for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("{:?}", self);
        let token = process_jwt_sign(self.sub.as_str(), self.aud.as_str(), self.exp.as_str())?;
        println!("{:?}", token);
        Ok(())
    }
}

impl CmdExecutor for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("{:?}", self);
        let v = process_jwt_verify(self.token)?;
        println!("{:?}", v);
        Ok(())
    }
}
