# tyg (type-gen)

[![Crates.io](https://img.shields.io/crates/v/tyg?style=flat-square)](https://crates.io/crates/tyg)
[![CI](https://img.shields.io/github/actions/workflow/status/SirCesarium/tyg/ci.yml?branch=main&style=flat-square)](https://github.com/SirCesarium/tyg/actions/workflows/ci.yml)
[![License](https://img.shields.io/github/license/SirCesarium/tyg?style=flat-square)]()

Generate types from several data sources to multiple languages.

```bash
tyg data.json                              # → Rust structs
tyg schema.yaml --lang typescript           # → TypeScript interface
curl https://api.example.com/data | tyg     # stdin works too
tyg -u https://api.a.com,https://api.b.com  # comma-separated URLs
zipcrawl *.zip cat config.json -q | tyg       # types from files inside ZIPs
tyg a.json b.json --lang rust > models.rs   # merge multiple samples
```

## How it works

Feed it one or more samples (files, URLs, stdin, any mix). It parses them, merges everything into a single shape, and generates the most specific type that covers all inputs. Multiple values (JSON streaming, multi-document YAML with `---`) merge the same way.

Format auto-detected from file extension. Override with `--format`.

### Input formats

| Format | Notes |
|--------|-------|
| JSON | Streaming multiple values merges |
| YAML | Multi-document (`---`) merges |
| TOML | Single document |
| XML | `@attr` for attributes, `#text` for content |
| .properties | Booleans and numbers auto-parsed |

### Output languages

| Language | Flag |
|----------|------|
| Rust | `--lang rust` |
| TypeScript interface | `--lang typescript` |
| TypeScript type alias | `--lang typescript-type-alias` |
| Kotlin (Jackson) | `--lang kotlin-jackson` |
| Kotlin (kotlinx) | `--lang kotlin-kotlinx` |
| JSON Schema | `--lang json-schema` |
| Shape | `--lang shape` |

## Install

```bash
cargo install tyg
```

## Options

| Flag | What |
|------|------|
| `sources` | File paths |
| `-u, --url` | Remote URLs (comma-separated: `-u url1,url2`) |
| `-f, --format` | Force: `json`, `yaml`, `toml`, `xml`, `properties` |
| `-n, --name` | Root type name (default: `Root`) |
| `-l, --lang` | Target: `rust`, `typescript`, `typescript-type-alias`, `kotlin-jackson`, `kotlin-kotlinx`, `json-schema`, `shape` |

## Unix philosophy

Pipe data from anywhere — like [zipcrawl](https://github.com/SirCesarium/zipcrawl) to extract types from ZIP contents without extraction:

```bash
zipcrawl releases/*.zip cat config.yaml -q | tyg --lang typescript
```

## License

MIT
