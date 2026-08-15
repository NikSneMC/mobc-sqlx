{inputs, ...}: {
  imports = [
    ./shell.nix
  ];

  perSystem = {system, ...}: {
    _module.args = let
      pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [inputs.naersk.inputs.fenix.overlays.default];
      };

      toolchain = pkgs.fenix.fromToolchainFile {
        file = ../rust-toolchain.toml;
        # sha256 = pkgs.lib.fakeHash;
        sha256 = "sha256-Du6MVMrLsqbYhnqdyenK/pNt1Fu24vNsiqPiW03a/Dg=";
      };

      naersk = pkgs.callPackage inputs.naersk {
        cargo = toolchain;
        rustc = toolchain;
      };
    in {
      inherit pkgs toolchain naersk;
    };
  };
}
