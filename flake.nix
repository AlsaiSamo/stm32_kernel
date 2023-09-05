{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "utils";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils, rust-overlay, crane }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.pkgsBuildHost.rust-bin.stable.latest.default.override {
          targets = [ "thumbv7em-none-eabihf" ];
          extensions = [ "llvm-tools-preview" "rust-src" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain rust;
        crate_expr = { stdenv, lib, qemu }:
          craneLib.buildPackage {
            src = craneLib.cleanCargoSource (craneLib.path ./.);
            depsBuildBuild = [ qemu ];
            nativeBuildInputs = [ ];
            buildInputs = [ ];
            CARGO_TARGET_THUMBV7EM_NONE_EABINF_LINKER =
              "${stdenv.cc.targetPrefix}cc";
            CARGO_TARGET_THUMBV7EM_NONE_EABINF_RUNNER = "qemu-system-arm";
            cargoExtraArgs = "--target thumbv7em-none-eabihf";
            doCheck = false;
          };
        crate = pkgs.callPackage crate_expr { };
      in {
        defaultPackage = crate;
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [ qemu openocd rust cargo-binutils gdb rust-analyzer ];
          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          NO_RUSTUP = 1;
          #what is this for?
          #LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib64:$LD_LIBRARY_PATH";
          #shellHook = ''
          #unset OBJCOPY
          #unset OBJDUMP
          #''
        };
      });
}
