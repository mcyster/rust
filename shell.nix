{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
    rustup
  ];

  buildInputs = with pkgs; [
    # sdl2
    #SDL2
    #SDL2.dev

    # bevy
    udev alsa-lib vulkan-loader
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
    libxkbcommon wayland # To use the wayland feature
  ];

  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}

