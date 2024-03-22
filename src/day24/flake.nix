{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, ... }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    python = (pkgs.python311.withPackages (pythonPackages:
      with pythonPackages; [
        python-lsp-server
      ]
    ));
  in {
    devShells.${system}.default = pkgs.mkShell {
      propagatedBuildInputs = with pkgs; [
        python
        # python311Packages.python-lsp-server
      ];

      shellHook = ''
        zsh
      '';
    };
  };
}
# vim: sw=2 ts=2
