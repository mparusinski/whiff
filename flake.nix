{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    # Flake-utils allows us to easily support multiple architectures.
    flake-utils.url = "github:numtide/flake-utils";
    # Naersk is a zero-configuration zero-codegen solution to packaging Rust.
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk { };

        # Here we can add non-rust dependencies that our program requires *at run time*.
        buildInputs = with pkgs; [

        ];

        # here we can add non-rust dependencies that our program requires *at build time*.
        nativeBuildInputs = with pkgs; [

        ];
      in
      rec {
        # Build this with `nix build`, run it with `nix run`
        defaultPackage = packages.app;
        packages =
          {
            app = naersk'.buildPackage {
              # Naersk will look for a `Cargo.toml` in this directory
              src = ./.;
              # Our buildinputs from above are specified here
              nativeBuildInputs = nativeBuildInputs;
              buildInputs = buildInputs;
            };
          };

        # This will be entered by direnv, or by manually running `nix shell`. This ensures
        # that our development environment will have all the correct tools at the correct
        # version for this project.
        devShell = pkgs.mkShell {
          # Here we add any tools that we want in our dev-shell but aren't required to build
          # our application.
          nativeBuildInputs = with pkgs;
            [
              nixpkgs-fmt
              cmake
              rustc
              cargo
              clippy
            ] ++ buildInputs ++ nativeBuildInputs;
          # The above line merges our buildInputs into the devshell, so we have them when
          # using cargo tools from inside our devshell.
        };
      }
    );
}
