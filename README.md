# mcp-ping

A simple MCP (Model Context Protocol) server written in Rust with a single `ping` tool that responds with "pong".

## Installation

### From GitHub Releases

Download the appropriate `.mcpb` package for your platform from the [Releases](https://github.com/your-username/mcp-dep/releases) page.

### From Source

```bash
git clone https://github.com/your-username/mcp-dep.git
cd mcp-dep
cargo build --release
```

The binary will be at `target/release/mcp-ping`.

## Usage

### With Claude Desktop

Add to your Claude Desktop configuration (`~/.config/claude/claude_desktop_config.json` on Linux/macOS or `%APPDATA%\Claude\claude_desktop_config.json` on Windows):

```json
{
  "mcpServers": {
    "ping": {
      "command": "/path/to/mcp-ping"
    }
  }
}
```

### Manual Testing

The server communicates over stdio using JSON-RPC. You can test it by running:

```bash
./target/release/mcp-ping
```

Then send JSON-RPC requests via stdin:

```json
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0.0"}}}
```

## Tools

| Tool | Description |
|------|-------------|
| `ping` | A simple ping tool that responds with "pong" |

## Development

### Prerequisites

- Rust 1.70 or later

### Building

```bash
cargo build
```

### Running

```bash
cargo run
```

### Testing

```bash
cargo test
```

## Release Process

Releases are automated via GitHub Actions. To create a new release:

1. Update the version in `Cargo.toml`
2. Commit and push changes
3. Create and push a version tag:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

The workflow builds binaries for:
- macOS (x86_64, aarch64)
- Linux (x86_64, aarch64)
- Windows (x86_64)

Each release includes:
- Platform-specific `.mcpb` packages
- `registry.json` for package discovery

## License

MIT License - see [LICENSE](LICENSE) for details.
