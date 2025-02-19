{ pkgs ? import <nixpkgs> { } }:
let
  system = pkgs.system;
  systemBuildInputs = system: pkgs:
    [ pkgs.iconv pkgs.openssl ]
    ++ (if pkgs.lib.strings.hasInfix "$system" "darwin" then [
      pkgs.darwin.apple_sdk.frameworks.Security
      pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
    ] else [ ]);
in with pkgs;
mkShell {
  shellHook = "export PATH=${"$"}PATH:~/.cargo/bin;";
  buildInputs = [
    rustup
    rustfmt
    cargo-make
    probe-rs-tools
    minicom
    just
    #nrf-command-line-tools
    # pre-commit
    # rustPackages.clippy
    flip-link

    # for rmkit
    bzip2 
  ] ++ (systemBuildInputs system pkgs);
  RUST_SRC_PATH = rustPlatform.rustLibSrc;
}

