use std::process::Command;


pub fn run_py_command(args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("./env/bin/python")
        .args(args)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Command failed with {}: {}", output.status, stderr).into());
    }
    
    Ok(String::from_utf8_lossy(&output.stdout).into())    
}
