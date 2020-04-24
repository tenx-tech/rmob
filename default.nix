{
  sources ? import ./nix/sources.nix,
  system ? builtins.currentSystem,
  crossSystem ? null,
  overlays ? [],
}:
let
  inherit (sources) nixpkgs nixpkgsMozilla cargo2nix;

  pkgs = import nixpkgs {
    inherit system crossSystem;
    overlays =
      let
        rustOverlay = import "${nixpkgsMozilla}/rust-overlay.nix";
        cargo2nixOverlay = import "${cargo2nix}/overlay";
      in
        overlays ++ [ cargo2nixOverlay rustOverlay ];
  };

  rustPkgs = pkgs.rustBuilder.makePackageSet' {
    rustChannel = "1.37.0";
    packageFun = import ./Cargo.nix;
    localPatterns = [
      ''^(src)(/.*)?'' # Integration test in `tests/` doesn't work in sandbox.
      ''[^/]*\.(rs|toml)$''
    ];
  };
in
  rec {
    inherit rustPkgs;
    package = rustPkgs.workspace.rmob {};
    ci = pkgs.rustBuilder.runTests rustPkgs.workspace.rmob { depsBuildBuild = [ pkgs.git ]; };
    shell = pkgs.mkShell {
      inputsFrom = pkgs.lib.mapAttrsToList (_: pkg: pkg {}) rustPkgs.noBuild.workspace;
      nativeBuildInputs = with rustPkgs; [ cargo rustc ];
    };
  }
