mod check_os;
mod windows;

fn main() {
    let os = check_os::detect_os();
    println!("Hello, world!");
    println!("OS: {}", os);
    windows::commands_base::commands_base().expect("Failed to execute commards_base");
}
