use std::env;
use std::io::{self, Read, Write};

const CHUNK_SIZE: usize = 16 * 1024; // 16 KiloBytes

fn main() {
    let silent = matches!(
        match env::var("PV_SILENT") {
            Ok(flag) => flag,
            Err(_) => "false".to_string(),
        }
        .as_str(),
        "1" | "true" | "on"
    );

    let mut total_bytes = 0;
    loop {
        let mut buffer = [0; CHUNK_SIZE];
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(e) => {
                if !silent {
                    eprintln!("Halted at error: {}", e);
                }
                break;
            }
        };
        io::stdout().write_all(&buffer[..num_read]).unwrap();
        total_bytes += num_read;
    }
    if !silent {
        eprintln!("total bytes read: {}", total_bytes);
    }
}
