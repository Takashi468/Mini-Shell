use std::env;

pub fn commands_base() -> std::io::Result<()> {
    let path = env::current_dir()?;
    let path = path.to_string_lossy().replace('\\', r"\");
    println!("Current directory: {}", path);
    Ok(())
}
