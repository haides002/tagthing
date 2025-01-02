{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
    bacon

    pkg-config

    exempi
  ];
}
