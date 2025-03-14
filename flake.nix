{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
          };
        in
        {
          devShell = pkgs.mkShell
            {
              LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib";

              buildInputs = with pkgs; [
                kind
                bashInteractive
                glibcLocales
                uv
                python311Packages.tox
                kubernetes-helm
                patchutils
              ];
            };
        }
      );
}
