{ inputs, ... }:
let
  rustChannel = "nightly";
  rustVersion = "latest";
  rustProfile = "default";
in
{
  imports = [ ./overlays.nix ];
  perSystem =
    {
      pkgs,
      craneLib,
      commonArgs,
      ...
    }:
    {
      _module.args = {
        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain (
          p: p.rust-bin.${rustChannel}.${rustVersion}.${rustProfile}
        );
        commonArgs = {
          # Depending on your code base, you may have to customize the
          # source filtering to include non-standard files during the build.
          # See
          # https://crane.dev/source-filtering.html?highlight=source#source-filtering
          src = craneLib.cleanCargoSource (craneLib.path ./.);

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            systemd
          ];
        };
      };

      # Build the executable package.
      packages.default = craneLib.buildPackage (
        commonArgs
        // {
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        }
      );

      devShells.rust = craneLib.devShell {
        packages =
          (commonArgs.nativeBuildInputs or [ ])
          ++ (commonArgs.buildInputs or [ ])
          ++ (with pkgs; [
            bacon
            rust-analyzer-unwrapped
          ]);

        RUST_SRC_PATH = "${
          pkgs.rust-bin.${rustChannel}.${rustVersion}.rust-src
        }/lib/rustlib/src/rust/library";
      };

      treefmt = {
        projectRootFile = "Cargo.toml";
        programs = {
          actionlint.enable = true;
          nixfmt.enable = true;
          rustfmt.enable = true;
        };
      };
    };
}
