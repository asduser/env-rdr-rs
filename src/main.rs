mod creds;

use std::{
    io::{self, BufWriter, Write},
    process::ExitCode,
};

fn print_values(c: &creds::Creds) -> io::Result<()> {
    let mut out = BufWriter::new(io::stdout().lock());
    writeln!(out, "Loaded {} values:", c.len())?;
    for (k, v) in c.iter_redacted() {
        writeln!(out, "{k}={v}")?;
    }
    out.flush()
}

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

    if let Err(e) = print_values(&c) {
        if e.kind() == io::ErrorKind::BrokenPipe {
            return ExitCode::SUCCESS;
        }
        eprintln!("error: {e}");
        return ExitCode::FAILURE;
    }

    match c.get("APP_GATEWAY") {
        Some(gw) => println!("Gateway: {gw}"),
        None => println!("APP_GATEWAY not set"),
    }
    ExitCode::SUCCESS
}
