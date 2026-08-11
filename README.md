# Wireguard Conf

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/idkncc/wireguard-conf/build.yml)](https://github.com/idkncc/wireguard-conf/actions)
[![docs.rs](https://img.shields.io/docsrs/wireguard-conf)](https://docs.rs/wireguard-conf)
[![Crates.io Version](https://img.shields.io/crates/v/wireguard-conf)](https://crates.io/crates/wireguard-conf)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/wireguard-conf)](https://crates.io/crates/wireguard-conf)
[![Quickstart Tutorial](https://img.shields.io/badge/Quickstart_Tutorial-blue)](./QUICKSTART.md)

Easy to use library for creating wireguard configs.

## Installation

Install `wireguard-conf`

```shell
cargo add wireguard-conf
```

### Usage

The best way to understand library: [Quickstart](./QUICKSTART.md)!

More usage examples you can find in [tests/tutorial.rs](tests/tutorial.rs), [tests folder](tests/) and documentation examples on [docs.rs](https://docs.rs/wireguard-conf).

```rust
use wireguard_conf::prelude::*;
use wireguard_conf::as_ipnet;

let peer = Peer::builder()
    .allowed_ips([as_ipnet!("10.0.0.2/24")])
    .build();

let interface = Interface::builder()
    .address([as_ipnet!("10.0.0.1/24")])
    .peers([peer.clone()])
    .build();

// to export configs, use `println!()`, `writeln!()`, `.to_string()`, etc.

println!("Server's config:");
println!("{}\n", interface);

println!("Client's config:");
println!("{}", peer.to_interface(&interface, ToInterfaceOptions::new()).unwrap());
```

### Features

- `amneziawg-*`: AmneziaWG is a fork of wireguard, that less recognizable by DPI systems by
   randomizing packet headers with packet sizes.

   These feature flags add support for generating and manually building amnezia settings:

   - `amneziawg`: _alias for `amneziawg-1`, this will update in next major release_
   - `amneziawg-1`: adds [AmneziaWG 1.0](https://docs.amnezia.org/documentation/amnezia-wg/) support.
   - `amneziawg-2`: adds [AmneziaWG 2.0](https://docs.amnezia.org/documentation/amnezia-wg/) support. **(recommended over `amneziawg-1`)**
   - ~~`amneziawg-3`~~: **WIP** (waiting for official release)

- `serde`: adds implementions of `serde::Serialize` and `serde::Deserialize` for all structs.

### Contributing

1. Fork & clone
2. Install Rust, Cargo and [just](https://just.systems/). 

   Or start nix shell: `direnv allow` or just `nix develop`
3. Make changes
4. Format and lint code:
   ```shell
   just fmt
   just lint
   # or fix automatically: just lint-fix
   ```
5. Commit changes (use [Conventional commits](https://www.conventionalcommits.org/en/v1.0.0/))
   ```shell
   git commit -m "feat: did something"
   ```
6. Send ~patches~ PR
