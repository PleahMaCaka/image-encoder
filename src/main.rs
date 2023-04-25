use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    let input: &str = &args[0];

    let exe_name: &str = {
        if input.contains("\\") {
            input.split("\\").last().unwrap()
        } else if input.contains("/") {
            input.split("/").last().unwrap()
        } else {
            input
        }
    };

    if args.len() == 1 {
        return println!(
            "usage:
            {} <options?> <file>
            -d --data-url :: add prefix for data url like `data:image/png;base64`
            -e --encode :: encode type :: [Default: Base64]
            -c --copy :: copy to clipboard
            -o --out --output :: output to file
            -p --print :: print to stdout
        ",
            exe_name
        );
    }

}
