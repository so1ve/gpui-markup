{ inputs, lib, ... }:

{
  imports = [
    (inputs.ray-devenv + "/profiles/config.nix")
    (inputs.ray-devenv + "/profiles/rust.nix")
  ]
  ++ lib.optional (builtins.pathExists ./devenv.local.nix) ./devenv.local.nix;
}
