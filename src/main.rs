use std::env::args;
use std::fs;
use std::io::Write;

use arboard::Clipboard;
use base64::engine::general_purpose;
use base64::Engine;

struct Config {
    data_url: bool,
    encode_type: String,
    copy: bool,
    output: String,
    print: bool,
    debug: bool,
    target: String,
}

fn main() {
    let mut args: Vec<String> = args().collect();

    let input: String = args.remove(0);

    let exe_name: &str = {
        if input.contains('\\') {
            input.split('\\').last().unwrap()
        } else if input.contains('/') {
            input.split('/').last().unwrap()
        } else {
            &input
        }
    };

    if args.is_empty()
        || args.contains(&String::from("-h"))
        || args.contains(&String::from("--help"))
    {
        return println!(
            "usage:
            {} <options?> <file>
            -d --data-url :: add prefix for data url like `data:image/png;base64`
            -e --encode :: encode type :: [Default: Base64]
                -- now support base64 only
            -c --copy :: copy to clipboard
            -o --out --output :: output to file
            -p --print :: print to stdout

            --debug :: debug this program
        ",
            exe_name
        );
    }

    let mut config = Config {
        data_url: false,
        encode_type: String::from("base64"),
        copy: false,
        output: String::new(),
        print: false,
        debug: false,
        target: String::new(),
    };

    // check args and update config
    // TODO check dir is valid
    let mut _count: usize = 0;
    let mut reference: Option<&mut String> = None;

    for arg in args {
        match reference {
            Some(r) => {
                *r = arg;
                reference = None;
            }
            None => {
                match arg.as_str() {
                    "-d" | "--data-url" => {
                        config.data_url = true;
                    }
                    "-e" | "--encode" => {
                        reference = Some(&mut config.encode_type);
                    }
                    "-c" | "--copy" => {
                        config.copy = true;
                    }
                    "-o" | "--out" | "--output" => {
                        reference = Some(&mut config.output);
                    }
                    "-p" | "--print" => {
                        config.print = true;
                    }
                    "--debug" => {
                        config.debug = true;
                    }
                    _ => {
                        config.target = arg;
                    }
                }
                _count += 1;
            }
        }
    }
    if config.debug {
        println!(
            "
        ^========== IMAGE ENCODER =========^
        data_url: {}        
        encode_type: {}
        copy: {}
        output: {}
        print: {}
        target: {}
        ",
            config.data_url,
            config.encode_type,
            config.copy,
            if config.output.is_empty() {
                "None"
            } else {
                config.output.as_str()
            },
            config.print,
            config.target
        );
    }

    if config.target.is_empty() {
        return println!("No target specified: {}", config.target);
    }

    let file = fs::read(&config.target).expect("Unable to read target file");

    let encoded: String = match config.encode_type.as_str() {
        "base64" => general_purpose::STANDARD.encode(file),
        _ => return println!("Unsupported encode type: {}", config.encode_type),
    };

    if config.print {
        println!("{}", encoded);
    }

    if config.copy {
        let mut clipboard = Clipboard::new().unwrap();
        if config.data_url {
            let mut data_url: String = String::from("data:image/");
            data_url.push_str(config.target.split('.').last().unwrap());
            data_url.push(';');
            data_url.push_str(config.encode_type.as_str());
            data_url.push(',');
            data_url.push_str(&encoded);
            clipboard.set_text(data_url).unwrap();
        } else {
            clipboard.set_text(&encoded).unwrap();
        }
    }

    if !config.output.is_empty() {
        let mut file = fs::File::create(config.output).expect("Unable to create output file");
        file.write_all(encoded.as_bytes())
            .expect("Unable to write to output file");
    }
}
