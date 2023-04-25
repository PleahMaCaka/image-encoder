# Introduction
This program is a cli utility that encodes files in base64. The encoded data can be copied to the clipboard, output to a file, or printed to standard output. The program also supports outputting in data URL format.

# Getting Started
```
$ cargo run -- <options?> <file>
```

`cargo run -- --help`

```
Usage:
    image-encoder.exe <options?> <file>
    -d --data-url :: Output in data URL format. e.g. `data:image/png;base64`
    -e --encode :: Specify encoding type. [Default: Base64]
        -- Currently only base64 is supported.
    -c --copy :: Copy to clipboard.
    -o --out --output :: Output to file.
    -p --print :: Print to standard output.

    --debug :: Enable debug mode for this program.
```

For more detailed information on how to use this program, please refer to the usage message printed when running the program with the `-h` or `--help` option.