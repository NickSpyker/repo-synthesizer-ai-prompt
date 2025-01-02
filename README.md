# Repo Synthesizer AI Prompt

A command-line tool that aggregates and formats repository content for AI prompts.
It walks through directories, collecting file contents while respecting ignore patterns, and outputs a formatted synthesis suitable for use in AI interactions.

## Features

- Walks through directories recursively to collect file contents
- Configurable ignore patterns for files, folders, and extensions
- Supports custom extension filtering (include or exclude)
- Outputs to file or stdout
- Customizable through a configuration file
- Cross-platform support (Windows, macOS, Linux)

## Installation

### Windows


### From Source

```shell
git clone https://github.com/NickSpyker/repo-synthesizer-ai-prompt.git
cd repo-synthesizer-ai-prompt
cargo install --path .
```

## Usage

The basic command is `synt`. Here are some common usage examples:

```shell
# Process current directory and output to stdout
synt

# Process a specific directory
synt -d /path/to/directory

# Output to a file
synt -o output.txt

# Process only specific file extensions
synt -e rs toml json

# Ignore specific file extensions
synt -i exe dll obj

# Open config file in system's file explorer
synt -c
```

## Configuration

The tool uses a configuration file located in the system's config directory:

- Windows: `%APPDATA%\repo-synthesizer-ai-prompt\config\config.json`
- macOS: `~/Library/Application Support/repo-synthesizer-ai-prompt/config/config.json`
- Linux: `~/.config/repo-synthesizer-ai-prompt/config/config.json`

You can quickly access this file using:

```shell
synt -c
```

The configuration file contains three main sections:

```json
{
  "ignored_folders": ["node_modules", ".git", ...],
  "ignored_files": [".gitignore", ".env", ...],
  "ignored_extensions": ["exe", "dll", ...]
}
```

## CLI Options

```
Options:
  -d, --directory <DIRECTORY>      Specifies the directory to analyze. If not provided, uses the current working directory
  -o, --output-file <OUTPUT_FILE>  Path to the output file. If not provided, prints to stdout
  -e, --extensions <EXTENSIONS>    List of file extensions to process. Only files with these extensions will be included
  -i, --ignore <IGNORE>            List of file extensions to ignore. Files with these extensions will be excluded
  -c, --config                     Opens the default config file in the system's file explorer
  -h, --help                       Print help
  -V, --version                    Print version
```

## Example Outputs

When processing a project, the tool generates output in this format:

```txt
src/main.rs:
[Content of main.rs]

----------------------------------------------------------------------------------------------------

src/lib.rs:
[Content of lib.rs]

----------------------------------------------------------------------------------------------------

Cargo.toml:
[Content of Cargo.toml]
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
