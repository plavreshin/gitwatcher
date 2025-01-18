# arcCloud metadata tracking API

# Table of Contents

0. [Prerequisites](#prerequisites)

## 0. Prerequisites

- Cargo

## Configuration

- Set `MISTRAL_API_KEY` environment variable to your Mistral API key for report command to work

## Implementation details

This project is just a command-line application that mainly has dependency on tokio and few others. It also bundles
mistral client api client library.

#### Sample run with gitwatcher report command

```sh
 INFO  gitwatcher > Generating report...
 INFO  gitwatcher > AI Analysis: ### Summary of Git Changes

```

As a result, one should have the following markdown file
generated [report_2025-01-18 11:14:13.md](reports/report_2025-01-18%2011%3A14%3A13.md)

### Sample run with gitwatcher watch command

```sh
 INFO  gitwatcher > Starting watch mode...
 INFO  gitwatcher > Listening for changes...
 INFO  gitwatcher > Found changes: [".gitignore", ".idea/vcs.xml", "Cargo.lock", "Cargo.toml", "README.md", "src/git_flow.rs", "src/main.rs", "src/mistral_analyzer.rs", "reports/"]
 INFO  gitwatcher > Listening for changes...
```

and it will be continuously listening for changes until you press Ctrl+C

### Project tree

Following is the project structure/tree layout:

```sh
[]()
[reports](reports)
[report_2025-01-18 11:14:13.md](reports/report_2025-01-18%2011%3A14%3A13.md)
[src](src)
[main.rs](src/main.rs)
[git_flow.rs](src/git_flow.rs)
[mistral_analyzer.rs](src/mistral_analyzer.rs)
[Cargo.lock](Cargo.lock)
[Cargo.toml](Cargo.toml)
[README.md](README.md)

```
