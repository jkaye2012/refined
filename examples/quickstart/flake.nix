{
  description = "Replace me";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.05";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    devenv = {
      url = "github:jkaye2012/devenv";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      fenix,
      nixpkgs,
      devenv,
      naersk,
    }:
    devenv.lib.forAllSystems nixpkgs (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        fenix' = fenix.packages.${system};
        naersk' = pkgs.callPackage naersk {
          cargo = fenix'.complete.toolchain;
          rustc = fenix'.complete.toolchain;
        };
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

        packages.${system}.default = naersk'.buildPackage {
          src = pkgs.nix-gitignore.gitignoreSource [ ] ./.;
        };
      }
    );
}
