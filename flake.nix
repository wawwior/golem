{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    inputs@{ self, ... }:
    let
      system = "x86_64-linux";
      pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [ inputs.rust-overlay.overlays.default ];
      };
      lib = pkgs.lib;
    in
    {

      devShells.${system}.default = pkgs.mkShell rec {

        buildInputs = with pkgs; [
          # Rust
          rust-bin.nightly."2025-03-18".default
          rust-analyzer
          wgsl-analyzer

          wayland # Wayland <- wgpu
          libxkbcommon # XKB <- winit
          libGL # GL <- wgpu
          vulkan-loader # Vulkan <- wgpu

        ];

        LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";

      };
    };
}
