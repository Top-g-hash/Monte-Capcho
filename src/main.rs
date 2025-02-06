use std::process::Command;
use chrono::Local;
use dirs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run slurp to capture a region
    let slurp_output = {
        let output = Command::new("slurp").output()?;
        if !output.status.success() {
            return Err("Error: slurp failed to capture a region.".into());
        }
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    };

    println!("Slurp output: {:?}", slurp_output);

    // Create the final save path in Pictures directory
    let pictures_dir = dirs::picture_dir()
        .ok_or("Could not find Pictures directory")?;
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let final_path = pictures_dir.join(format!("screenshot_{}.png", timestamp));

    // Capture the screenshot with grim directly using slurp output
    let status = Command::new("grim")
        .args(["-g", &slurp_output])
        .arg(&final_path)
        .status()?;

    if !status.success() {
        return Err(format!("Error: grim command failed with geometry: {}", slurp_output).into());
    }

    println!("Screenshot saved to: {}", final_path.display());
    Ok(())
}
