{
  outputs = {nixpkgs, ...}: let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
  in with pkgs; {
    devShells.x86_64-linux.default = mkShell rec {
      nativeBuildInputs = [
        pkg-config
      ];
      buildInputs =[
        udev
        alsa-lib
        vulkan-loader
        xorg.libX11
        xorg.libXcursor
        xorg.libXi
        xorg.libXrandr
        libxkbcommon
        wayland 

        # For lld linker
        clang
        lld
      ];
      LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
    };
  };
}
