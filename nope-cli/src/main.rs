use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds a new nope to the list
    Add {
        /// The language code (e.g., "en", "es")
        lang: String,

        /// The nope phrase
        nope: String,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Nope {
    language: String,
    nope: String,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { lang, nope } => {
            add_nope(lang, nope)?;
        }
    }

    Ok(())
}

fn add_nope(lang: &str, nope_phrase: &str) -> std::io::Result<()> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // Go up to the workspace root
    path.push("nopes.json");

    let file_content = fs::read_to_string(&path)?;
    let mut nopes: Vec<Nope> = serde_json::from_str(&file_content)?;

    // Check for duplicates
    if nopes.iter().any(|n| n.language == lang && n.nope == nope_phrase) {
        println!("Nope already exists.");
        return Ok(());
    }

    let new_nope = Nope {
        language: lang.to_string(),
        nope: nope_phrase.to_string(),
    };

    nopes.push(new_nope);

    // Sort by language for consistency
    nopes.sort_by(|a, b| a.language.cmp(&b.language));

    let updated_json = serde_json::to_string_pretty(&nopes)?;
    fs::write(&path, updated_json)?;

    println!("Successfully added new nope: '{}' for language '{}'", nope_phrase, lang);

    Ok(())
}
