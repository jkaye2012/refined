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
        crane-stable = (crane.mkLib pkgs).overrideToolchain fenix'.stable.minimalToolchain;
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

        testFeature =
          feature:
          crane'.buildPackage {
            inherit src;
            cargoTestExtraArgs = "--no-default-features --all-targets --features ${feature}";
          };

        refined = crane'.buildPackage {
          inherit src;
          cargoTestExtraArgs = "--all-features";
        };

        refined-stable = crane-stable.buildPackage {
          inherit src;
          cargoTestExtraArgs = "--lib";
        };

        refined-no-std = crane'.buildPackage {
          inherit src;
          cargoTestExtraArgs = "--no-default-features --all-targets";
        };

        refined-doc = crane'.cargoDoc {
          inherit src;
          cargoArtifacts = refined;
          cargoDocExtraArgs = "--all-features";
        };

        refined-example-quickstart = buildExample {
          inherit src;
          subdir = "examples/quickstart";
          args = {
            cargoArtifacts = refined;
          };
        };

        refined-example-optimized = buildExample {
          inherit src;
          subdir = "examples/optimized";
          args = {
            cargoArtifacts = refined;
          };
        };

        refined-example-axum = buildExample {
          inherit src;
          subdir = "examples/axum";
          args = {
            cargoArtifacts = refined;
          };
        };
      in
      {
        devShells.${system}.default = pkgs.mkShell {
          inherit (manifest) name;

          inputsFrom = [ devenv.devShells.${system}.default ];

          packages = with pkgs; [
            cargo-show-asm
            fenix'.complete.toolchain
            linuxPackages_latest.perf
            lldb
          ];

          RUSTDOCFLAGS = "--cfg docsrs";
        };

        checks.${system} = {
          inherit
            refined
            refined-stable
            refined-no-std
            refined-doc
            refined-example-axum
            refined-example-quickstart
            refined-example-optimized
            ;
          refined-test-serde = testFeature "serde";
          refined-test-alloc = testFeature "alloc";
          refined-test-std = testFeature "std";
          refined-test-optimized = testFeature "optimized";
          refined-test-implication = testFeature "implication";
          refined-test-arithmetic = testFeature "arithmetic";
          refined-test-regex = testFeature "regex";
          refined-test-full = testFeature "full";
        };

        packages.${system} = rec {
          inherit
            refined
            refined-doc
            refined-example-axum
            refined-example-quickstart
            refined-example-optimized
            ;
          default = refined;
        };
      }
    );
}
