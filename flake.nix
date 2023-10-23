{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
    ravedude.url = "github:Rahix/avr-hal?dir=ravedude";
  };
  outputs = { self, nixpkgs, rust-overlay, ravedude }: 
  let
    system = "x86_64-linux";
    src = ./.;
    name = "keep-close";

    overlays = [ (import rust-overlay) ];
    pkgs = import nixpkgs {
      inherit system overlays;
    };
    rustToolchain = (
      pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.minimal.override {
        extensions = [ "rust-src" ];
      }));
  in
  {
    devShells.${system}.default = with pkgs; mkShell {
      buildInputs = [
        rustToolchain
        ravedude.packages.${system}.default
        pkgsCross.avr.buildPackages.gcc
        avrdude
      ];
    };
  };
}
