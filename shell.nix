{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    geckodriver
  ];

  shellHook = ''
    if ! lsof -i :4444 | read; then
        echo "geckodriver not yet running on port 4444, executing \`geckodriver &\`.."
        geckodriver &
    fi
  '';
}

