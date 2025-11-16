use anyhow::Result;
use clap::Parser;
use todolist::cli::args::Cli;
use todolist::core::store::TodoManager;
use todolist::ui::display::display_error;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut manager = match TodoManager::new() {
        Ok(mgr) => mgr,
        Err(e) => {
            eprintln!("Failed to initialize todo manager: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = cli.execute(&mut manager) {
        display_error(&format!("Error: {}", e));
        std::process::exit(1);
    }
    Ok(())
}
