//! Easy to use Wireguard config generator.
//!
//! - Use [`InterfaceBuilder`] and [`PeerBuilder`] for interface/peers creation.
//! - Use [`Interface`]'s and [`Peer`]'s [`std::fmt::Display`] for exporting  Wireguard config (`.to_string()`, [`write!()`], etc).
//! - Use [`PrivateKey`], [`PublicKey`] and [`PresharedKey`] for generating, importing and
//!   exporting keys.
//! - Use [`AmneziaSettings`] for generating/using AmneziaWG obfuscation values.
//!
//! # Features
//!
//! - `amneziawg-*`: AmneziaWG is a fork of wireguard, that less recognizable by DPI systems by
//!    randomizing packet headers with packet sizes.
//! 
//!    These feature flags add support for generating and manually building amnezia settings:
//! 
//!    - `amneziawg`: _alias for `amneziawg-1`, this will update in next major release_
//!    - `amneziawg-1`: adds [AmneziaWG 1.0](https://docs.amnezia.org/documentation/amnezia-wg/) support.
//!    - `amneziawg-2`: adds [AmneziaWG 2.0](https://docs.amnezia.org/documentation/amnezia-wg/) support. **(recommended over `amneziawg-1`)**
//!    - ~~`amneziawg-3`~~: **WIP** (waiting for official release)
//! 
//! - `serde`: adds implementions of [`serde::Serialize`] and [`serde::Deserialize`] for all structs.
//!
//! # Example
//!
//! ```rust
//! use wireguard_conf::prelude::*;
//! use wireguard_conf::as_ipnet;
//!
//! let peer = Peer::builder()
//!     .allowed_ips([as_ipnet!("10.0.0.2/24")])
//!     .build();
//!
//! let interface = Interface::builder()
//!     .address([as_ipnet!("10.0.0.1/24")])
//!     .peers([peer.clone()])
//!     .build();
//!
//! // to export configs, use `println!()`, `writeln!()`, `.to_string()`, etc.
//!
//! println!("Server's config:");
//! println!("{}\n", interface);
//!
//! println!("Client's config:");
//! println!("{}", peer.to_interface(&interface, ToInterfaceOptions::new()).unwrap());
//! ```

#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod macros;
mod models;
mod utils;

pub mod prelude;

pub use ipnet;

pub use models::*;
pub use utils::*;
