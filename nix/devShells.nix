{
  imports = [
    ./overlays.nix
    ./rust.nix
  ];
  perSystem =
    {
      self',
      pkgs,
      ...
    }:
    {
      devShells.default = self'.devShells.rust;
    };
}
