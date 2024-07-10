# vfox Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/vfox)](https://pkg.fluentci.io/vfox)
[![ci](https://github.com/fluentci-io/vfox-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/vfox-plugin/actions/workflows/ci.yml)

A Fluent CI plugin to setup [vfox](https://vfox.lhan.me/).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm vfox setup
```

## Functions

| Name    | Description                                            |
| ------- | ------------------------------------------------------ |
| setup   | Download and install vfox                              |
| install | Install a tool version [aliases: i]                    |
| use     | Install tool version and add it to config [aliases: u] |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.2.1"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/vfox@v0.1.0?wasm=1", "setup", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: vfox
    args: |
      setup
```
