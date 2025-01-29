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

        src = crane'.cleanCargoSource ./.;

        buildExample =
          {
            src,
            subdir,
            args ? { },
          }:
          crane'.buildPackage {
            inherit src;
            cargoLock = ./${subdir}/Cargo.lock;
            cargoToml = ./${subdir}/Cargo.toml;

            postUnpack = ''
              cd $sourceRoot/${subdir}
              sourceRoot="."
            '';
          }
          // args;

        refined = crane'.buildPackage {
          inherit src;
          cargoTestExtraArgs = "--all-features";
        };

        refined-doc = crane'.cargoDoc {
          inherit src;
          cargoArtifacts = refined;
          cargoDocExtraArgs = "--all-features";
        };

        refined-example-quickstart = buildExample {
          inherit src;
          subdir = "examples/quickstart";
        };

        refined-example-axum = buildExample {
          inherit src;
          subdir = "examples/axum";
        };
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

        checks.${system} = {
          inherit
            refined
            refined-doc
            refined-example-axum
            refined-example-quickstart
            ;
        };

        packages.${system} = rec {
          inherit
            refined
            refined-doc
            refined-example-axum
            refined-example-quickstart
            ;
          default = refined;
        };
      }
    );
}
