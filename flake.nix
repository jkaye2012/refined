{
  description = "Simple refinement types for Rust with Serde support";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.11";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    devenv = {
      url = "github:jkaye2012/devenv";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      fenix,
      nixpkgs,
      devenv,
      crane,
    }:
    devenv.lib.forAllSystems nixpkgs (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        fenix' = fenix.packages.${system};
        crane' = (crane.mkLib pkgs).overrideToolchain fenix'.complete.toolchain;
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
      in
      {
        devShells.${system}.default = pkgs.mkShell {
          inherit (manifest) name;

          inputsFrom = [ devenv.devShells.${system}.default ];

          packages = with pkgs; [
            fenix'.complete.toolchain
            linuxPackages_latest.perf
            lldb
          ];
        };

        # TODO: ensure that docs and examples can be built
        packages.${system}.default = crane'.buildPackage {
          src = crane'.cleanCargoSource ./.;
          cargoTestExtraArgs = "--all-features";
        };
      }
    );
}
