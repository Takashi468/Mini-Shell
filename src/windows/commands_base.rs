use std::env;
use std::path::Path;
use crate::print_input::print_input;

pub fn commands_base(input: &str) -> std::io::Result<()> {
    // let path = env::current_dir()?;
    // let path = path.to_string_lossy().replace('\\', r"\");
    // println!("Current directory: {}", path);
    // Ok(())
    let input_lower = input.to_lowercase();

    let args: Vec<String> = input_lower
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    if args.is_empty(){
        return Ok(());
    }

    match args.as_slice() {
        [command, args @ ..] => {
            match command.as_str() {
                "pwd" => {
                    let path = env::current_dir()?;
                    let path = path.to_string_lossy().replace('\\', r"\");
                    println!("{}",path);
                    Ok(())
                }
                // "cd" => {
                //    let root = Path::new("/");
                //    let args = args.join(" ");
                //    let path = Path::new(&args);
                //    let full_path = root.join(&path);
                //    env::set_current_dir(&full_path)?;
                //    println!("Changed directory to: {}", full_path.to_string_lossy().replace('\\', r"\\"));
                //    Ok(())
                // }
                "cd" => {
                    let target_path = if args.is_empty(){
                        match env::var("HOME") {
                            Ok(home) => std::path::PathBuf::from(home),
                            Err(_) => {
                                eprintln!("cd: HOME not set");
                                return Ok(());
                            }
                        }
                    }else {
                        std::path::PathBuf::from(&args[0])
                    };

                    match env::set_current_dir(&target_path) {
                        Ok(_) => {
                            println!("Changed directory to: {}", target_path.to_string_lossy().replace('\\', r"\\"));
                            Ok(())
                        }
                        Err(e) => {
                            eprintln!("cd: {}", e);
                            Ok(())
                        }
                    }
                }
                "print" => {
                    println!("args, {:#?}", args);
                    if let Some(first_arg) = args.get(0){
                        print_input(&first_arg.to_string());
                    }else{
                        println!("Error: No arguments provided to print!")
                    }
                    // let print_input = if !args.is_empty(){
                    //     print_input(&args[0].to_string());
                    // }else{
                    //     println!("what ?");
                    // };
                    Ok(())
                }
                "echo" => {
                    if let Some(first_arg) = args.get(0) {
                        println!("{}", first_arg);
                    } else {
                        println!("Error: No arguments provided to echo!");
                    }
                    Ok(())
                }
                _ => {
                    println!("Unknown command: {}", command);
                    Ok(())
                }
            }
        }
        _ => {
            println!("No command provided");
            Ok(())
        }
    }
}
