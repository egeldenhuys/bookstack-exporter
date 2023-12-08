# BookStack Exporter
Export a BookStack instance as a hierarchy of files.

Currently, this tool is very simple and does not offer a lot of customisations or error checking.
Check out [homeylab/bookstack-file-exporter](https://github.com/homeylab/bookstack-file-exporter) to see if it will meet your needs.

## Features
- No additional dependencies
- Runs on Windows and Linux
- Export all pages from BookStack while keeping the structure
  - Uses shelve/book/chapter/page slug for naming

### Missing Features
- [ ] Download attachments
- [ ] Rewrite links to make the html export browsable offline

## Usage
```
Usage: bookstack-exporter [OPTIONS]

Options:
      --host <BOOKSTACK_HOST>
          Bookstack Host. Example: docs.example.com

  -e, --export-type <EXPORT_TYPE>
          Type of export to perform. Required unless set in the config file

          [possible values: html, pdf, markdown]

  -o, --output-dir <OUTPUT_DIR>
          Directory to export Bookstack to

  -i, --api-id <BOOKSTACK_API_TOKEN_ID>
          Bookstack API Token ID

          Can also be set with the environment variable BOOKSTACK_API_TOKEN_ID

  -s, --api-secret <BOOKSTACK_API_TOKEN_SECRET>
          Bookstack API Token Secret

          Can also be set with the environment variable BOOKSTACK_API_TOKEN_SECRET

  -c, --config-path <CONFIG_PATH>
          Optional config file

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

Examples:

    Load all config values from a custom config file path,
    and override export type to use the command line argument instead

    $ bookstack-exporter --config-path settings.toml --export-type pdf

    settings.toml:
        bookstack_host = "https://docs.example.com"
        output_dir = "export"
        export_type = "html"
        bookstack_api_token_id = "<token_id>"
        bookstack_api_token_secret = "<token_secret>"
```

### Config file
Example config file:
```toml
bookstack_host = "https://docs.example.com"
output_dir = "export"
export_type = "html"
bookstack_api_token_id = "<token_id>"
bookstack_api_token_secret = "<token_secret>"
```

### Example output
```
.
└── export
    ├── my-shelf-1
    │         └── a-book
    │             ├── a-chapter-on-stuff
    │             │         └── this-page-in-a-chapter.hml
    │             └── some-page.html
    └── my-shelf-2
        └── another-book
            └── another-page.html
```

## Install
Download the latest binary for your platform from the [Releases](https://github.com/egeldenhuys/bookstack-exporter/releases) page.

## Build
You will need to set up a [Rust](https://www.rust-lang.org/) development environment to edit and build this project.

Alternatively, you can fork this repo and start a [GitHub Codespace](https://github.com/features/codespaces) to use a preconfigured development environment.

To build bookstack-exporter:
```
$ cargo build --release
```

### Cross compile for Windows from Linux

```bash
$ cargo install cargo-xwin
$ rustup target add x86_64-pc-windows-msvc.
$ cargo xwin build --release --xwin-arch x86_64 --target x86_64-pc-windows-msvc
```

## Contributing
This project tries to apply the following conventions:
- [keep a changelog](https://keepachangelog.com/en/1.1.0/)
- [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)

## TODO
- [ ] Sanitize file paths
  - Only allow exporting into given directory
- [ ] Handle cases where directory and files already exist
- [ ] Cleanup code
- [ ] Error handling
- [ ] Tests
