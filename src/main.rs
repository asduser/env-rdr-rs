mod creds;

use std::process::ExitCode;

fn main() -> ExitCode {
    let c = match creds::load_creds() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };

    if c.is_empty() {
        eprintln!("error: values store is empty");
        return ExitCode::FAILURE;
    }

    println!("Loaded values: {c:?}");

    match c.get("APP_GATEWAY") {
        Some(gw) => println!("Gateway: {gw}"),
        None => println!("APP_GATEWAY not set"),
    }
    ExitCode::SUCCESS
}
