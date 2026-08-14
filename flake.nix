{
  description = "wipctl dev shell (toolchain from rust-toolchain.toml)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  # This flake provides a development shell and nothing else: no packages, no
  # apps, no checks. Every gate lives in `.pre-commit-config.yaml` and the
  # `justfile`; the flake's only job is to pin the toolchain those gates run on.
  outputs =
    { nixpkgs, rust-overlay, ... }:
    let
      # Inlined instead of flake-utils.lib.eachDefaultSystem: one fewer input to
      # lock, and flake-utils has been unmaintained since 2024-11.
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems =
        f:
        nixpkgs.lib.genAttrs systems (
          system:
          f (
            import nixpkgs {
              inherit system;
              overlays = [ (import rust-overlay) ];
            }
          )
        );
    in
    {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = [
            # Reads channel + components + targets straight from
            # rust-toolchain.toml, which stays the single home for the version.
            (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)

            # Rust quality gates: `cargo fmt --check` and `cargo clippy` come
            # from the toolchain's components above; these are the rest.
            pkgs.cargo-deny
            pkgs.cargo-audit
            pkgs.cargo-nextest
            pkgs.cargo-machete # unused-dependency gate, pre-push stage

            # Gate entry points. The justfile is the single caller of every
            # check; pre-commit is the commit- and push-stage gate.
            pkgs.just
            pkgs.pre-commit

            # `language: system` pre-commit hooks get no environment of their
            # own and resolve off PATH, so the devShell is the only thing that
            # can supply them.
            pkgs.taplo # TOML formatting (Cargo.toml, deny.toml, taplo.toml)
            pkgs.dprint # JSON + markdown formatting
            pkgs.shellcheck # install.sh / uninstall.sh / install-common.sh
            pkgs.shfmt # the same shell payload, formatting side
            pkgs.typos # spell check; the upstream hook ids do not run here
            pkgs.committed # commit-message shape, same reason
            pkgs.gawk # scripts/check-emphasis, scripts/check-adr-length
            pkgs.ripsecrets # secret scan, pre-commit stage
            pkgs.gitleaks # secret scan, pre-push stage
            pkgs.editorconfig-checker # charset / EOL / trailing whitespace

            # Nix quality tools for this flake itself — the repository owes its
            # own toolchain a gate for every artifact it ships, this one
            # included (docs/reference/quality-gates.md).
            pkgs.nixfmt # RFC 166 formatter; nixfmt-rfc-style is a deprecated alias
            pkgs.statix
            pkgs.deadnix

            # markdownlint-cli2 stays on pre-commit's own node env — its
            # `markdownlint-rule-relative-links` rule has no nixpkgs
            # derivation, and `additional_dependencies` is the only thing that
            # can install it, so converting the hook would silently drop the
            # relative-links check. Its `language_version: system` then sources
            # node from HERE; absent this package pre-commit downloads a
            # generic-glibc node whose ELF interpreter
            # (/lib64/ld-linux-x86-64.so.2) does not exist on a Nix host.
            pkgs.nodejs

            # Release path to crates.io.
            pkgs.release-plz
            pkgs.cargo-dist # invoked as `dist`

            # The record is kept in git and the transition commit is specified
            # against it (ADR-0036); the journey test drives a real checkout.
            pkgs.git
          ];
          # native deps for -sys crates, uncomment as needed:
          # buildInputs = [ pkgs.openssl ];
          # nativeBuildInputs = [ pkgs.pkg-config ];
          shellHook = ''echo "wipctl dev shell ready (rust $(rustc --version | cut -d' ' -f2))"'';
        };
      });
    };
}
