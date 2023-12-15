{
  description = "sounce - Sound Synthesizer written in Rust";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    devshell = {
      url = "github:mora-unie-youer/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = inputs:
    inputs.flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.devshell.overlays.default
            inputs.rust-overlay.overlays.default
          ];
        };
      in {
        # Default devshell for Sounce development
        devShells.default = pkgs.devShell.mkShell {
          name = "sounce";
          packages = with pkgs; [
            # Toolchain required for C + Rust binaries building
            binutils
            gcc

            # Packages required for building crates
            pkg-config
            alsa-lib
            libjack2

            # Nightly Rust toolchain
            bacon
            cargo-flamegraph
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = [ "rust-analyzer" "rust-src" ];
            }))
          ];
        };
      }
    );
}
