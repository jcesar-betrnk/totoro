#![deny(warnings)]

use clap::{Parser, Subcommand};
use clipboard_rs::{Clipboard, ClipboardContext};
use directories::ProjectDirs;
use google_authenticator::GoogleAuthenticator;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use toml::{Table, Value};

const QUAL: &str = "com";
const ORG: &str = "ivanceras";
const APP: &str = env!("CARGO_PKG_NAME");

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
    let mut filename = config_dir.to_path_buf();
    filename.set_extension("toml");
    Ok(filename)
}

fn write_to_clipboard(content: &str) -> anyhow::Result<()> {
    let ctx = ClipboardContext::new().expect("Could not get access clipboard");
    ctx.set_text(content.to_string())
        .expect("Could not set the text in the clipboard");
    // ISSUE: it seems it need to be read here in order to make it work
    let _clip = ctx.get_text().expect("Could not read the clipboard text");
    Ok(())
}

fn read_toml_table() -> anyhow::Result<Table> {
    let filename = config_file()?;
    if let Ok(toml_content) = fs::read_to_string(&filename) {
        let toml_value: Result<Value, _> = toml::from_str(&toml_content);
        let Ok(Value::Table(table)) = toml_value else {
            panic!("expecting valid key value toml format");
        };
        Ok(table)
    } else {
        Ok(Table::new())
    }
}

fn ensure_config_dir_exists() -> anyhow::Result<()>{
    let config_file = config_file()?;
    let prefix = config_file.parent().expect("must have a parent directory for config file");
    match fs::create_dir_all(prefix){
        Ok(_) => Ok(()),
        Err(_) => {
            panic!("Unable to create directory: {}", prefix.display());
        }
    }
}

fn save_table_to_toml(table: &Table) -> anyhow::Result<()> {
    let content = toml::to_string(table).unwrap();
    let config_file = config_file()?;
    ensure_config_dir_exists()?;
    let mut file = fs::File::create(config_file)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn copy_totp_to_clipboard(domain: &str) -> anyhow::Result<()> {
    let table = read_toml_table()?;
    let value = table.get(domain);
    match value {
        Some(value) => {
            let Value::String(secret) = value else {
                panic!("must be a string");
            };
            let auth = GoogleAuthenticator::new();
            let code = auth.get_code(&secret, 0).unwrap();
            write_to_clipboard(&code)?;
            println!("{code}");
        }
        None => {
            println!("There is no such domain");
        }
    }
    Ok(())
}

fn add_totp_entry(domain: &str, secret: &str) -> anyhow::Result<()> {
    let mut table = read_toml_table()?;
    table.insert(domain.to_string(), secret.into());
    save_table_to_toml(&table)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
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
