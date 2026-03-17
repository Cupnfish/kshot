use clap::Parser;
use kscreenshot::{CaptureRequest, ScreenCaptureManager};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "kshot")]
#[command(about = "Windows screenshot CLI using Windows Graphics Capture", long_about = None)]
struct Args {
    /// Output file path
    #[arg(short, long, default_value = "screenshot.png")]
    output: PathBuf,

    /// Screen index to capture (0-based). If not specified, captures primary screen.
    #[arg(short, long)]
    screen: Option<usize>,

    /// List all screens and exit
    #[arg(short, long)]
    list: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut manager = ScreenCaptureManager::new()?;
    manager.set_capture_method_name("WGC")?;

    // List screens mode
    if args.list {
        let screens = manager.list_screens()?;
        for (i, screen) in screens.iter().enumerate() {
            let is_primary = screen.id == manager.primary_screen()?.id;
            println!(
                "[{}] {}x{}{}",
                i,
                screen.width,
                screen.height,
                if is_primary { " (primary)" } else { "" }
            );
        }
        return Ok(());
    }

    // Determine which screen to capture
    let target_screen = if let Some(idx) = args.screen {
        let screens = manager.list_screens()?;
        screens.get(idx).cloned().ok_or_else(|| {
            format!("Screen index {} out of range (0-{})", idx, screens.len() - 1)
        })?
    } else {
        manager.primary_screen()?
    };

    let request = CaptureRequest::screen(target_screen.id);
    let result = manager.capture(request)?;
    result.source.save(&args.output)?;

    println!("Screenshot saved to: {}", args.output.display());

    Ok(())
}
