// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "rstu")]
struct Cli {
    /// Log parsing progress to stderr.
    #[arg(long, global = true)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Version,
    Check {
        file: String,
    },
    Format {
        file: String,
        #[arg(long)]
        output: Option<OutputFormat>,
    },
    Convert {
        file: String,
        #[arg(short, long, value_enum, default_value = "json")]
        output: OutputFormat,
        #[arg(long, value_name = "BOOL", action = clap::ArgAction::Set)]
        prefer_block_scalars: Option<bool>,
    },
}

#[derive(Clone, Debug, ValueEnum)]
enum OutputFormat {
    JSON,
    YAML,
}

fn main() {
    let cli = Cli::parse();

    if cli.debug {
        env_logger::Builder::new()
            .filter_module("rstu_parser", log::LevelFilter::Debug)
            .format_timestamp(None)
            .format_target(false)
            .init();
    }

    match cli.command {
        Commands::Version => {
            println!("rstu {}", env!("CARGO_PKG_VERSION"));
        }
        Commands::Check { file } => {
            println!("Subcommand check, option file={file}");
        }
        Commands::Format { file, output } => {
            let option_value = output
                .map(|value| format!("{value:?}").to_lowercase())
                .unwrap_or_else(|| "none".to_string());

            println!("Subcommand format, option file={file}, output={option_value}");
        }
        Commands::Convert {
            file,
            output,
            prefer_block_scalars,
        } => {
            if prefer_block_scalars.is_some() && !matches!(output, OutputFormat::YAML) {
                eprintln!("error: --prefer-block-scalars requires --output yaml");
                std::process::exit(2);
            }

            let input = std::fs::read_to_string(&file).unwrap_or_else(|error| {
                eprintln!("error: cannot read {file}: {error}");
                std::process::exit(1);
            });

            match rstu_parser::parse(&input) {
                Ok(document) => {
                    let serialized = match output {
                        OutputFormat::JSON => {
                            serde_json::to_string(&rstu_ast::AstNode::to_json(&document))
                                .expect("AST serialization should not fail")
                        }
                        OutputFormat::YAML => rstu_ast::AstNode::to_yaml_with_options(
                            &document,
                            prefer_block_scalars.unwrap_or(true),
                        )
                        .unwrap_or_else(|error| {
                            eprintln!("error: failed to serialize {file}: {error}");
                            std::process::exit(1);
                        }),
                    };
                    print!("{serialized}");
                    if !serialized.ends_with('\n') {
                        println!();
                    }
                }
                Err(error) => {
                    eprintln!("error: failed to parse {file}: {error}");
                    std::process::exit(1);
                }
            }
        }
    }
}
