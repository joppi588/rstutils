// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "rstu")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Check {
        file: String,
    },
    Format {
        file: String,
        #[arg(long)]
        output: Option<OutputFormat>,
    },
    Parse {
        file: String,
    },
}

#[derive(Clone, Debug, ValueEnum)]
enum OutputFormat {
    Json,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { file } => {
            println!("Subcommand check, option file={file}");
        }
        Commands::Format { file, output } => {
            let option_value = output
                .map(|value| format!("{value:?}").to_lowercase())
                .unwrap_or_else(|| "none".to_string());

            println!("Subcommand format, option file={file}, output={option_value}");
        }
        Commands::Parse { file } => {
            let input = std::fs::read_to_string(&file).unwrap_or_else(|error| {
                eprintln!("error: cannot read {file}: {error}");
                std::process::exit(1);
            });

            match rstu_parser::parse(&input) {
                Ok(document) => {
                    let json = serde_json::to_string(&rstu_ast::AstNode::to_json(&document))
                        .expect("AST serialization should not fail");
                    println!("{json}");
                }
                Err(error) => {
                    eprintln!("error: failed to parse {file}: {error:?}");
                    std::process::exit(1);
                }
            }
        }
    }
}
