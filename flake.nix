{
  description = "FFI bindings for Hunspell & Nuspell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { nixpkgs, rust-overlay, ... }:
    let
      inherit (nixpkgs) lib;
      forEachSystem = lib.genAttrs lib.systems.flakeExposed;
    in
    {
      devShell = forEachSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        pkgs.mkShell rec {
          nativeBuildInputs = with pkgs; [
            (toolchain.override {
              extensions = [
                "rust-src"
                "clippy"
                "llvm-tools-preview"
                "miri"
              ];
            })
            rust-analyzer
            cargo-flamegraph
            cargo-llvm-cov
            gcc
            clang-tools
            cmake
            gnumake
            libcxx
            icu74
            gdb
            valgrind
          ];
          CPATH = lib.makeSearchPathOutput "dev" "include" nativeBuildInputs;
          LD_LIBRARY_PATH = lib.makeLibraryPath [pkgs.stdenv.cc.cc.lib pkgs.icu74];
          RUST_BACKTRACE = "1";
        }
      );
    };
}
