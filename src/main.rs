mod git_flow;
mod mistral_analyzer;

use chrono::Local;
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use mistral_analyzer::MistralAnalyzer;

const DEFAULT_REPO_PATH: &str = "/Users/plavreshin/Projects/taltech/gitwatcher";
const DEFAULT_INTERVAL_IN_SECONDS: u64 = 3;

const DEFAULT_WATCH_COMMAND: Commands = Commands::Watch {
    path: None,
    interval: DEFAULT_INTERVAL_IN_SECONDS,
};

const DEFAULT_REPORT_COMMAND: Commands = Commands::Report { path: None };

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Watch {
        #[arg(short, long)]
        path: Option<String>,

        #[arg(short, long, default_value_t = DEFAULT_INTERVAL_IN_SECONDS)]
        interval: u64,
    },
    Report {
        #[arg(short, long)]
        path: Option<String>,
    },
}

fn get_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let analyzer = MistralAnalyzer::new().expect("Failed to initialize Mistral analyzer");
    let cli = Cli::parse();

    match cli.command.unwrap_or(DEFAULT_WATCH_COMMAND) {
        Commands::Watch { path, interval } => {
            let repo_path = path.clone().unwrap_or_else(|| {
                std::env::var("REPO_PATH").unwrap_or(DEFAULT_REPO_PATH.to_string())
            });

            if !std::path::Path::new(&repo_path).join(".git").exists() {
                log::error!(
                    "Repo path does not exist or is not a Git repository: {}",
                    repo_path
                );
                return;
            }

            let git_flow = git_flow::GitFlow::new(repo_path);

            log::info!("Starting watch mode...");
            loop {
                log::info!("Listening for changes...");
                match git_flow.get_changed_files() {
                    Ok(changed_files) => {
                        log::info!("Found changes: {:?}", changed_files);
                    }
                    Err(e) => {
                        log::error!("Failed to get changed files: {}", e);
                    }
                }
                tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
            }
        }
        Commands::Report { path } => {
            let repo_path = path.clone().unwrap_or_else(|| {
                std::env::var("REPO_PATH").unwrap_or(DEFAULT_REPO_PATH.to_string())
            });

            let git_flow = git_flow::GitFlow::new(repo_path);
            log::info!("Generating report...");

            match git_flow.get_diff_as_text() {
                Ok(changed_files) => {
                    if !changed_files.is_empty() {
                        match analyzer.analyze_changes(changed_files).await {
                            Ok(analysis) => {
                                log::info!("AI Analysis: {}", analysis);
                                let report_dir = std::path::Path::new("reports");
                                if !report_dir.exists() {
                                    std::fs::create_dir(report_dir)
                                        .expect("Failed to create reports directory");
                                }
                                let report_path =
                                    report_dir.join(format!("report_{}.md", get_time()));
                                std::fs::write(
                                    &report_path,
                                    format!(
                                        "# Git Changes Analysis Report\n\nGenerated: {}\n\n{}",
                                        get_time(),
                                        analysis
                                    ),
                                )
                                .expect("Failed to write report file");
                                log::info!("Report saved to: {:?}", report_path);
                            }
                            Err(e) => log::error!("Failed to analyze changes: {}", e),
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to generate report: {}", e);
                }
            }
        }
    }
}
