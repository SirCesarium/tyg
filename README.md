# 🏭 Type-Forge

A fast CLI tool to transform JSON, YAML, TOML, and XML into type-safe code for Rust, TypeScript, and more.

**Type-Forge** is a data-to-code pipeline that ingest structured data from multiple sources and "type-forges" it into usable language models. It is designed to be the bridge between raw configuration/API responses and type-safe implementation.

## Features

* **Multi-Format Ingestion**: Native support for JSON, YAML, TOML, XML, and Java `.properties`.
* **Source Agnostic**: Read from local files, fetch from remote URLs, or consume from `stdin`.
* **Unified Model Generation**: Merges multiple samples (via multiple file arguments or JSON streaming) into a single optimized type definition.
* **Polyglot Output**: Generate code for Rust, TypeScript (Classes or Type Aliases), Kotlin (Jackson or kotlinx.serialization), and JSON Schema.
* **Auto-Detection**: Smart format detection based on file extensions.

## Installation

```bash
# Clone the repository
git clone https://github.com/SirCesarium/type-forge
cd type-forge

# Build and install
cargo install --path .

```

## Usage

### Transform local files

Type-Forge detects the format and generates Rust structs by default.

```bash
type-forge config.yaml settings.toml

```

### Fetch from Remote APIs

Specify the format if the URL doesn't provide it.

```bash
type-forge --url "https://api.example.com/data" --format json --lang typescript

```

### Pipe from [ZipCrawl](https://github.com/SirCesarium/zipcrawl)

Analyze files inside compressed archives without extraction.

```bash
zipcrawl archive.zip cat internal_data.xml | type-forge --format xml --name DataModel

```

### Advanced Merging

Ingest multiple sources to generate a type that satisfies all samples.

```bash
type-forge sample1.json sample2.json --lang rust > models.rs

```

## Commands & Options

| Argument | Description | Options |
| --- | --- | --- |
| `sources` | List of local file paths | `file.json`, `config.toml`, etc. |
| `-u, --url` | Remote data source | Any valid URL |
| `-f, --format` | Force input format | `json`, `yaml`, `toml`, `xml`, `properties` |
| `-n, --name` | Set root type name | Default: `Root` |
| `-l, --lang` | Target language | `rust`, `typescript`, `kotlin`, `json_schema` |

## Target Modes

| Mode | Alias | Output Type |
| --- | --- | --- |
| **Rust** | `rust` | Serde-compatible Structs |
| **TypeScript** | `typescript` | Classes |
| **TS Alias** | `typescript/typealias` | Type Definitions |
| **Kotlin** | `kotlin/jackson` | Jackson-annotated Classes |
| **JSON Schema** | `json_schema` | Standard JSON Schema v7 |
