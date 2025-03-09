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
        LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
                libGL
                libxkbcommon
                wayland
        ];

}
