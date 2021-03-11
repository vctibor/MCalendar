# Use latest stable Nix channel
{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-20.03.tar.gz") { } }:

pkgs.mkShell {
  nativeBuildInputs = [ pkgs.pkg-config ];
  buildInputs = [
    pkgs.libxml2
    pkgs.libxslt
    pkgs.openssl
    pkgs.postgresql
    pkgs.readline
    pkgs.zlib
    pkgs.binutils.bintools
  ];
}