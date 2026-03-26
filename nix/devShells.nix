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
      devShells.default = pkgs.mkShell {
        name = "development shell";
        inputsFrom = [
          self'.devShells.rust
        ];
      };
    };
}
