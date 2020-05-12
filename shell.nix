with (import <nixpkgs> {});
let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  ruststable = (nixpkgs.latest.rustChannels.stable.rust.override { extensions = [ "rust-src" "rls-preview" "rust-analysis" "rustfmt-preview" ];});
  basePackages = [ openssl nasm rustup ruststable cmake zlib gnumake gcc readline openssl libxml2 curl];

  inputs = if system == "x86_64-darwin" then
              basePackages ++ [darwin.apple_sdk.frameworks.CoreServices]
           else
              basePackages;
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "pndev";
    buildInputs = inputs;

    shellHook = ''
        export OPENSSL_DIR="${nixpkgs.openssl.dev}"
        export OPENSSL_LIB_DIR="${nixpkgs.openssl.out}/lib"
    '';
  }
