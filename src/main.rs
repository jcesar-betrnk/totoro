use clap::{Parser, Subcommand};
use clipboard_rs::{Clipboard, ClipboardContext, ContentFormat};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;
use toml::Value;
use clipboard_rs::ClipboardContextX11Options;
use totp_rs::{Secret, TOTP, Rfc6238, Algorithm};
use toml::Table;
use base32;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::time;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;

const QUAL: &str = "com";
const ORG: &str = "ivanceras";
const APP: &str = "pastilan";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new entry totp for a domain with the specified base32 secret code
    Add {
        /// which domain name
        #[arg(short, long)]
        domain: String,
        /// specify the totp secret
        #[arg(short, long)]
        secret: String,
    },
    /// Get a totp digits for the specified domain
    Get {
        /// which domain name
        #[arg(short, long)]
        domain: String,
    },
}

fn config_file() -> anyhow::Result<PathBuf> {
    let proj_dirs = ProjectDirs::from(QUAL, ORG, APP).expect("Could not open config file");
    let config_dir = proj_dirs.config_dir();
    dbg!(&config_dir);
    let mut filename = config_dir.to_path_buf();
    dbg!(&filename);
    filename.set_extension("toml");
    dbg!(&filename);
    Ok(filename)
}

fn write_to_clipboard(content: &str) -> anyhow::Result<()> {
    println!("setting clipboard with content: {}",content);
    let ctx = ClipboardContext::new().expect("Could not get access clipboard");
    ctx.set_text(content.to_string())
        .expect("Could not set the text in the clipboard");
    // ISSUE: it seems it need to be read here in order to make it work
    let clip = ctx.get_text().expect("Could not read the clipboard text");
    dbg!(clip);
    Ok(())
}

fn read_toml_table() -> anyhow::Result<Table> {
    let filename = config_file()?;
    let toml_content = fs::read_to_string(&filename)?;
    dbg!(&toml_content);
    let toml_value: Result<Value, _> = toml::from_str(&toml_content);
    dbg!(&toml_value);
    let Ok(Value::Table(table)) = toml_value else {
        panic!("expecting valid key value toml format");
    };
    dbg!(&table);
    Ok(table)
}

fn save_table_to_toml(table: &Table) -> anyhow::Result<()>{
    let content = toml::to_string(table).unwrap();
    dbg!(&content);
    let config_file = config_file()?;
    fs::write(config_file, content)?;
    Ok(())
}

fn decode_secret_key(secret: &str) -> anyhow::Result<Vec<u8>> {
    if let Some(secret_bytes) = base32::decode(base32::Alphabet::Rfc4648 { padding: false }, secret){
        Ok(secret_bytes)
    }else{
        println!("the base32 decoding failed, trying base64..");
        let secret_bytes = BASE64_STANDARD.decode(secret)?;
        Ok(secret_bytes)
    }
}

fn derive_totp(secret: &str, digits: u32, epoch: u64, interval: u64) -> Result<u64, &'static str> {
    let secret_bytes = base32::decode(base32::Alphabet::Rfc4648 { padding: false }, secret)
        .ok_or("Invalid base32")?;

    //let secret_bytes = decode_secret_key(secret).unwrap();

    let mut hmac: Hmac<Sha1> =
        Mac::new_from_slice(&secret_bytes).expect("HMAC should take any length");
    hmac.update(
        &((time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            - epoch)
            / interval)
            .to_be_bytes(),
    );
    let result = hmac.finalize().into_bytes();
    let offset = (result[19] & 0b1111) as usize;
    Ok(
        (u32::from_be_bytes(result[offset..offset + 4].try_into().unwrap()) as u64)
            % 10u64.pow(digits),
    )
}

fn copy_totp_to_clipboard(domain: &str) -> anyhow::Result<()> {
    let table = read_toml_table()?;
    let value = table.get(domain);
    dbg!(&value);
    match value {
        Some(value) => {
            let Value::String(secret) = value else {
                panic!("must be a string");
            };
            //let rfc = Rfc6238::with_defaults(secret.as_bytes().to_vec())?;
            //let totp = TOTP::from_rfc6238(rfc)?;

            //let secret = Secret::Encoded(secret.to_string());
            //let totp = TOTP::new_unchecked(Algorithm::SHA1, 6, 1, 30, secret.to_bytes().unwrap());

            let digits = 6;
            let epoch = 0;
            let interval = 30;

            let code = derive_totp(secret, digits, epoch, interval).unwrap();
            println!("code: {code}");
            //println!("digits: {:digits$}", digits = code as usize);

            //let code = totp.generate_current()?;
            //println!("code: {code}");
            let code = code.to_string();
            write_to_clipboard(&code)?;
        }
        None => {
            println!("There is no such domain");
        }
    }
    Ok(())
}

fn add_totp_entry(domain: &str, secret: &str) -> anyhow::Result<()>{
    let mut table = read_toml_table()?;
    table.insert(domain.to_string(), secret.into());
    save_table_to_toml(&table)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    dbg!(&args);
    match args.command {
        Commands::Add { domain, secret } => {
            add_totp_entry(&domain, &secret)?;
        }
        Commands::Get { domain } => {
            copy_totp_to_clipboard(&domain)?;
        }
    }

    Ok(())
}
