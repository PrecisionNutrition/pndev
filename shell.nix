# to update the nix-channel run `nix-shell -p niv --run 'niv update'`
{ sources ? import ./nix/sources.nix }:
with (import sources.nixpkgs { });
let
  basePackages =
    [ cargo clippy gcc openssl pkg-config readline rustc rustfmt rustup zlib ];

  inputs = if stdenv.isDarwin then
    basePackages ++ [ darwin.apple_sdk.frameworks.CoreServices darwin.apple_sdk.frameworks.Security ]
  else
    basePackages;

in pkgs.mkShell {
  buildInputs = inputs;

  # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela.
  RUST_SRC_PATH = "${rust.packages.stable.rustPlatform.rustLibSrc}";
}
