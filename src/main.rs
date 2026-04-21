mod ai;
mod compression;
mod smugness;

use clap::{Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[command(
    name = "middle-out",
    about = "🚀 MIDDLEOUT™ — AI-Powered Compression Technology",
    long_about = concat!(
        "\n",
        "  ⚠️  DISCLAIMER: This is joke software.\n",
        "  It does not compress files. It makes them 10% LARGER.\n",
        "  Do not use in production. Do not use at all, probably.\n",
        "\n",
        "MIDDLEOUT™ uses a proprietary middle-out algorithm to achieve\n",
        "unprecedented compression ratios* by identifying the optimal\n",
        "Weissman pivot and expanding around it.\n",
        "\n",
        "  * 'Compression ratio' here means how much bigger we made it.\n",
    ),
    version = "1.0.0-revolutionary"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// AI API endpoint (OpenAI-compatible) for personalized compression commentary
    #[arg(long, global = true, env = "MIDDLEOUT_AI_ENDPOINT")]
    ai_endpoint: Option<String>,

    /// API key for the AI endpoint
    #[arg(long, global = true, env = "MIDDLEOUT_API_KEY")]
    api_key: Option<String>,

    /// AI model to use for commentary
    #[arg(long, global = true, env = "MIDDLEOUT_MODEL", default_value = "gpt-4o-mini")]
    model: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Compress a file using revolutionary MIDDLEOUT™ technology (makes it bigger)
    Compress {
        /// Input file to compress
        input: PathBuf,
        /// Output file (defaults to <input>.moc)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Decompress a MIDDLEOUT™ .moc file (undoes the damage)
    Decompress {
        /// Input .moc file
        input: PathBuf,
        /// Output file (defaults to <input> without .moc)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Show statistics for a compressed .moc file
    Stats {
        /// The .moc file to inspect
        input: PathBuf,
    },
    /// Explain what MIDDLEOUT™ actually does (be honest for once)
    Explain,
}

fn main() {
    let cli = Cli::parse();
    smugness::print_banner();

    match cli.command {
        Commands::Compress { input, output } => {
            let output = output.unwrap_or_else(|| {
                let mut p = input.clone();
                let ext = match p.extension() {
                    Some(e) => format!("{}.moc", e.to_string_lossy()),
                    None => "moc".to_string(),
                };
                p.set_extension(ext);
                p
            });

            println!("  {} {}", "Input:".bold(), input.display());
            println!("  {} {}", "Output:".bold(), output.display());
            println!();

            let pb = ProgressBar::new(100);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("  {msg}\n  [{bar:50.cyan/blue}] {pos}% {spinner}")
                    .unwrap()
                    .progress_chars("█▉▊▋▌▍▎▏ "),
            );

            // Theatrical progress display
            for (i, &line) in smugness::COMPRESSION_LINES.iter().enumerate() {
                pb.set_message(format!("{} {}", "⟳".cyan(), line));
                pb.set_position((i as u64 + 1) * 10);
                thread::sleep(Duration::from_millis(120));
            }
            pb.finish_and_clear();

            match compression::compress(&input, &output) {
                Ok((original, compressed)) => {
                    println!("  {} {}", "✓".green().bold(), smugness::random_success_line().white().bold());
                    smugness::print_smug_ratio(original, compressed);

                    if let Some(endpoint) = &cli.ai_endpoint {
                        println!("  {} Consulting AI for personalized commentary...", "🤖".cyan());
                        if let Some(commentary) = ai::get_ai_commentary(
                            endpoint,
                            cli.api_key.as_deref(),
                            &cli.model,
                            original,
                            compressed,
                            "compress",
                        ) {
                            ai::print_ai_commentary(&commentary);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("  {} Compression failed: {e}", "✗".red().bold());
                    eprintln!("  (The irony of our compression algorithm failing is not lost on us.)");
                    std::process::exit(1);
                }
            }
        }

        Commands::Decompress { input, output } => {
            let output = output.unwrap_or_else(|| {
                let s = input.to_string_lossy();
                // strip trailing .moc component from extension
                if s.ends_with(".moc") {
                    PathBuf::from(&s[..s.len() - 4])
                } else {
                    let mut p = input.clone();
                    p.set_extension("restored");
                    p
                }
            });

            println!("  {} {}", "Input:".bold(), input.display());
            println!("  {} {}", "Output:".bold(), output.display());
            println!();

            let pb = ProgressBar::new(100);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("  {msg}\n  [{bar:50.cyan/blue}] {pos}% {spinner}")
                    .unwrap()
                    .progress_chars("█▉▊▋▌▍▎▏ "),
            );
            pb.set_message(format!("{} Locating the middle...", "⟳".cyan()));
            pb.set_position(30);
            thread::sleep(Duration::from_millis(300));
            pb.set_message(format!("{} Surgically removing proprietary padding...", "⟳".cyan()));
            pb.set_position(70);
            thread::sleep(Duration::from_millis(300));
            pb.set_message(format!("{} Restoring original data integrity...", "⟳".cyan()));
            pb.set_position(100);
            thread::sleep(Duration::from_millis(200));
            pb.finish_and_clear();

            match compression::decompress(&input, &output) {
                Ok(original_size) => {
                    println!("  {} Decompression complete. The middle has been removed.", "✓".green().bold());
                    println!("  {} Restored size: {} bytes", "→".cyan(), original_size.to_string().white().bold());
                    println!();
                    println!("  {} We are deeply aware that we put it there in the first place.", "Note:".yellow().bold());
                    println!();

                    if let Some(endpoint) = &cli.ai_endpoint {
                        println!("  {} Consulting AI for post-decompression commentary...", "🤖".cyan());
                        if let Some(commentary) = ai::get_ai_commentary(
                            endpoint,
                            cli.api_key.as_deref(),
                            &cli.model,
                            original_size,
                            original_size,
                            "decompress",
                        ) {
                            ai::print_ai_commentary(&commentary);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("  {} Decompression failed: {e}", "✗".red().bold());
                    std::process::exit(1);
                }
            }
        }

        Commands::Stats { input } => {
            match compression::read_header(&input) {
                Ok(header) => {
                    let file_size = std::fs::metadata(&input).map(|m| m.len()).unwrap_or(0);
                    println!("  {} MIDDLEOUT™ File Analysis", "📊".cyan());
                    println!();
                    println!("  File:             {}", input.display());
                    println!("  Format version:   {}", compression::VERSION);
                    println!("  Original size:    {} bytes", header.original_size.to_string().white().bold());
                    println!("  Padding inserted: {} bytes {}", header.padding_size.to_string().red().bold(), "(the 'middle')".dimmed());
                    println!("  File size on disk:{} bytes", file_size.to_string().yellow().bold());
                    println!("  Overhead:         {} bytes (header)", compression::HEADER_SIZE);
                    println!();
                    println!("  {}", smugness::random_stats_line().italic().bright_black());
                    println!();

                    if let Some(endpoint) = &cli.ai_endpoint {
                        println!("  {} Requesting AI analysis...", "🤖".cyan());
                        if let Some(commentary) = ai::get_ai_commentary(
                            endpoint,
                            cli.api_key.as_deref(),
                            &cli.model,
                            header.original_size,
                            file_size,
                            "stats",
                        ) {
                            ai::print_ai_commentary(&commentary);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("  {} Could not read file: {e}", "✗".red().bold());
                    std::process::exit(1);
                }
            }
        }

        Commands::Explain => {
            println!("  {}", "The MIDDLEOUT™ Algorithm — An Honest Explanation".white().bold());
            println!();
            println!("  Q: Does MIDDLEOUT™ actually compress files?");
            println!("  A: {}", "No. Not even slightly.".red().bold());
            println!();
            println!("  Q: What does it do, exactly?");
            println!("  A: It finds the middle of your file and inserts random bytes.");
            println!("     Specifically, ~10% of the original file size in random padding.");
            println!("     Then it saves that — now 10% larger — as a .moc file.");
            println!();
            println!("  Q: Why would anyone do this?");
            println!("  A: {}", "Silicon Valley reference. Pied Piper. Middle-out.".italic());
            println!("     If you don't get it, the show is worth watching.");
            println!();
            println!("  Q: What about the 'AI-Powered' part?");
            println!("  A: If you pass --ai-endpoint, we ask the AI to tell you how");
            println!("     impressive the result is. The AI is complicit.");
            println!();
            println!("  Q: Is the Weissman Score™ real?");
            println!("  A: In the show, yes. Here, it's a random number near 6.28.");
            println!("     We chose 6.28 because it's 2π and felt appropriately pretentious.");
            println!();
            println!("  Q: Can I use this in production?");
            println!("  A: {}", "Please do not.".red().bold());
            println!();
            println!("  {}", "— The MIDDLEOUT™ Research Division".dimmed().italic());
            println!();
        }
    }
}
