# Seeded by release-kit: a starting point this project owns and tunes;
# release-kit reports drift here and never rewrites it.
#
# Supported shape: one crate with a [package] table — an implicit
# src/main.rs binary or an explicit [[bin]] entry — building with the
# committed Cargo.lock. A workspace root fails by name below rather than
# throwing on a missing attribute: point the importTOML call at the member
# crate's Cargo.toml and set mainProgram yourself.
{ lib, rustPlatform }:

let
  cargoToml = lib.importTOML ../Cargo.toml;
  package =
    cargoToml.package
      or (throw "nix/package.nix: Cargo.toml has no [package] table; this seed does not support a workspace root");
in
rustPlatform.buildRustPackage {
  pname = package.name;
  inherit (package) version;

  src = lib.cleanSource ../.;
  cargoLock.lockFile = ../Cargo.lock;

  # tests/docs_gates.rs runs the prose gates in scripts/ as child processes,
  # and their `#!/usr/bin/env bash` shebang does not resolve in the build
  # sandbox, which carries no /usr/bin/env. patchShebangs rewrites each one to
  # the store bash, so the gates run here exactly as they run in the devshell.
  postPatch = ''
    patchShebangs scripts
  '';

  meta = {
    # The first [[bin]] name where one is declared, else the package
    # name — the implicit src/main.rs binary. nix run resolves the
    # binary through this attribute.
    mainProgram = if cargoToml ? bin then (lib.head cargoToml.bin).name else package.name;
    # Cargo.toml states `MIT OR Apache-2.0`. A list is how Nix spells a
    # disjunction: a consumer may take either licence.
    license = with lib.licenses; [
      mit
      asl20
    ];
  }
  // lib.optionalAttrs (package ? description) { inherit (package) description; }
  // lib.optionalAttrs (package ? homepage) { inherit (package) homepage; };
}
