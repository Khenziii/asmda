{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    geckodriver
  ];

  shellHook = ''
    geckodriver --port 4444 &
  '';
}

