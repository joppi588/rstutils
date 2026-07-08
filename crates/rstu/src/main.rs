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
    }
}
