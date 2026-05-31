{
  description = "Corrosium OS Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "corrosium-os-dev";

          buildInputs = with pkgs; [
            rustup
            gnumake
            # Use qemu_kvm if available on Linux for better performance, fallback to qemu
            (if pkgs.stdenv.isLinux then qemu_kvm else qemu)
          ];

          shellHook = ''
            echo "========================================="
            echo " Corrosium OS Dev Environment            "
            echo "========================================="
            echo ""

            # Ensure the required toolchains and targets are installed via rustup
            # This allows seamless switching with `cargo +stable` and `cargo +nightly`,
            # or directory-specific rust-toolchain.toml files.

            echo "Verifying stable toolchain (x86_64-unknown-uefi)..."
            rustup toolchain install stable --profile minimal --target x86_64-unknown-uefi

            echo "Verifying nightly toolchain (x86_64-unknown-none)..."
            rustup toolchain install nightly --profile minimal \
              --component rustfmt,rust-src,clippy \
              --target x86_64-unknown-none

            echo ""
            echo "Environment ready! Tools available:"
            echo " - rustup, cargo, rustc"
            echo " - make (gnumake)"
            echo " - qemu"
          '';
        };
      }
    );
}
