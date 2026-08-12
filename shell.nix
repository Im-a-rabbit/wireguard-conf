{
  pkgs ? import <nixpkgs> { },
}:
let
  rust-toolchain = (fromTOML (builtins.readFile ./rust-toolchain.toml)).toolchain;
in
with pkgs;
mkShell {
  nativeBuildInputs = [
    rustup
    rustPlatform.bindgenHook
  ];

  RUSTC_VERSION = rust-toolchain.channel;

  shellHook = ''
    export PATH="''${CARGO_HOME:-~/.cargo}/bin":"$PATH"
    export PATH="''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-${stdenv.hostPlatform.rust.rustcTarget}/bin":"$PATH"
  '';
}
