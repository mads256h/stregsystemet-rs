{
    description = "Stregsystemet-rs nix flake";
    inputs = {
        nixpkgs.url = "nixpkgs/nixos-24.05";
    };
    outputs = { self, nixpkgs }: let 
        system = "x86_64-linux";
        pkgs = import nixpkgs {inherit system;};
    in {
        devShells.${system}.default = pkgs.mkShellNoCC {
            DATABASE_URL = "postgres://stregsystemet:password@localhost/stregsystemet";
            packages = with pkgs; [
                cargo
                rustc
                postgresql
            ];
        };
        packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
            name = "stregsystemet-rs";
            cargoSha256 = "sha256-VY0UXted3xbpoHTscFZ1gc/n8xHPU4JYjJeDpzUkC+4=";
            src = ./.;
        };
    };
}
