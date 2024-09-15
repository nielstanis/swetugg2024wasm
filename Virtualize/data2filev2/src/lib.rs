#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::cli::run::Guest;
use wasi_http_client::Client;
use std::env;
use std::fs;
use std::io::{Read, Write};

struct Component;

bindings::export!(Component with_types_in bindings);

impl Guest for Component {
    fn run() -> Result<(), ()> {

        let arguments = env::args().collect::<Vec<String>>();

        if arguments.len() < 3 {
            eprintln!("usage: {} <from> <to>", arguments[0]);
            return Ok(());
        }

        if arguments[1].starts_with("http"){
        
            if let Err(err) = processurl(&arguments[1], &arguments[2]) {
                eprintln!("{}", err)
            }
            return Ok(());
        }

        if let Err(err) = processfile(&arguments[1], &arguments[2]) {
            eprintln!("{}", err);
        }
        
        Ok(())
    }
}

fn processfile(input_fname: &str, output_fname: &str) -> Result<(), String> {
    let mut input_file =
        fs::File::open(input_fname).map_err(|err| format!("error opening input {}: {}", input_fname, err))?;
    let mut contents = Vec::new();
    input_file
        .read_to_end(&mut contents)
        .map_err(|err| format!("read error: {}", err))?;

    let mut output_file = fs::File::create(output_fname)
        .map_err(|err| format!("error opening output {}: {}", output_fname, err))?;
    output_file
        .write_all(&contents)
        .map_err(|err| format!("write error: {}", err))
}

fn processurl(input_url: &str, output_fname: &str) -> Result<(), String> {
    let resp = Client::new()
        .get(input_url)
        .send()
        .map_err(|err| format!("Unable to read url error: {}", err))?;

    eprint!("Fetched '{}' and got Status:{}", input_url, resp.status());
        
    let mut output_file = fs::File::create(output_fname)
        .map_err(|err| format!("error opening output {}: {}", output_fname, err))?;
    output_file
        .write_all(&resp.body())
        .map_err(|err| format!("write error: {}", err))
}