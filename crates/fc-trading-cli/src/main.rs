use std::env;
use std::io::{self, Write as _};
use std::path::PathBuf;
use std::time::Duration;

use clap::{Parser, Subcommand};
use color_eyre::eyre::{Context as _, Result, eyre};
use secrecy::SecretString;
use ssi_fc_trading::{
    ClientConfig, Credentials, StreamEvent, StreamOptions, TradingClient, TwoFactorType,
};
use zeroize::Zeroizing;

const DEFAULT_API_URL: &str = "https://fc-tradeapi.ssi.com.vn/";
const DEFAULT_STREAM_URL: &str = "https://fc-tradehub.ssi.com.vn/";

#[derive(Debug, Parser)]
#[command(
    name = "fctrading",
    version,
    about = "Safe diagnostics for SSI FastConnect Trading"
)]
struct Cli {
    #[arg(long, default_value = ".env")]
    env_file: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Doctor,
    RequestOtp,
    VerifyWrite,
    Stream {
        #[arg(long, default_value = "-1")]
        notify_id: String,
        #[arg(long, default_value_t = 30)]
        seconds: u64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    dotenvy::from_path(&cli.env_file)
        .with_context(|| format!("loading {}", cli.env_file.display()))?;
    let client = load_client()?;
    match cli.command {
        Command::Doctor => {
            let response = client.rate_limit().await?;
            write_line(format_args!(
                "FastConnect doctor succeeded: status={} message={}",
                response.status, response.message
            ))?;
        }
        Command::RequestOtp => {
            let response = client.request_otp().await?;
            write_line(format_args!(
                "OTP request completed: status={} message={}",
                response.status, response.message
            ))?;
        }
        Command::VerifyWrite => {
            let code = Zeroizing::new(rpassword::prompt_password("Verification code: ")?);
            client.verify_code(&code).await?;
            write_line(format_args!("Write authentication succeeded"))?;
        }
        Command::Stream { notify_id, seconds } => {
            let mut stream = client.stream(StreamOptions::new(notify_id)).await?;
            let deadline = tokio::time::Instant::now() + Duration::from_secs(seconds);
            loop {
                tokio::select! {
                    () = tokio::time::sleep_until(deadline) => break,
                    event = stream.next() => match event {
                        Some(Ok(StreamEvent::Broadcast(payload))) => {
                            write_line(format_args!("{payload}"))?;
                        }
                        Some(Ok(StreamEvent::ServerError(message))) => {
                            write_line(format_args!("server error: {message}"))?;
                        }
                        Some(Ok(_)) => {}
                        Some(Err(error)) => return Err(error.into()),
                        None => break,
                    }
                }
            }
            stream.close().await?;
            write_line(format_args!("Stream closed"))?;
        }
    }
    Ok(())
}

fn load_client() -> Result<TradingClient> {
    let api_url = env::var("SSI_FCTRADING_API_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_owned());
    let stream_url =
        env::var("SSI_FCTRADING_STREAM_URL").unwrap_or_else(|_| DEFAULT_STREAM_URL.to_owned());
    let config = ClientConfig::new(&api_url, &stream_url)?;
    let consumer_id = required_env("SSI_FCTRADING_CONSUMER_ID")?;
    let consumer_secret = SecretString::from(required_env("SSI_FCTRADING_CONSUMER_SECRET")?);
    let private_key = SecretString::from(required_env("SSI_FCTRADING_PRIVATE_KEY")?);
    let credentials = Credentials::from_base64_xml(consumer_id, consumer_secret, private_key)?;
    let two_factor = match env::var("SSI_FCTRADING_TWO_FACTOR_TYPE").as_deref() {
        Ok("1") => TwoFactorType::Otp,
        Ok("2") => TwoFactorType::Ca,
        Ok("0") | Err(_) => TwoFactorType::Pin,
        Ok(other) => return Err(eyre!("invalid SSI_FCTRADING_TWO_FACTOR_TYPE {other}")),
    };
    TradingClient::new(config, credentials, two_factor).map_err(Into::into)
}

fn required_env(name: &str) -> Result<String> {
    env::var(name).with_context(|| format!("missing required environment variable {name}"))
}

fn write_line(arguments: std::fmt::Arguments<'_>) -> Result<()> {
    let mut output = io::stdout().lock();
    writeln!(output, "{arguments}")?;
    Ok(())
}
