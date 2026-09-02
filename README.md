# wipctl

wipctl is a planning method and the tooling that proves a project's plan record obeys it, shipped together and judged as one. The method plans with bounded stories, judgment-counting points, five lanes, and recorded transitions. The tooling is one command, `wipctl`, that scaffolds a project's plan, answers what to start next, takes work atomically, validates the record, moves entries between lanes as recorded events, captures and lands new work, replicates the plan between machines, and renders the record as terminal views.

A project's plan lives in a plan repository of its own — a plain git repository holding five lane files, one document per story, epic, and initiative, a charter, open questions, a transition journal, and an on-demand capture directory — shared by every worktree and clone of the project on a machine, and hosted wherever its operator chooses. The project itself carries exactly one committed file, `.wipctl.toml`, naming its identity; that file is the entire footprint, and everything else belongs to the project.

## Install

Build and install the command from a checkout:

```bash
git clone https://github.com/gubasso/wipctl.git
cd wipctl
cargo install --path .
```

The command surface the binary implements is specified in [docs/reference/](./docs/reference/), which is where to look for the exact verbs, arguments, and exit codes rather than inferring them from the build.

## Development

Tooling is pinned in a Nix flake devShell, so a contributor gets the same toolchain the gates run against:

```bash
direnv allow   # loads the devShell on entering the directory
nix develop    # or enter it ad hoc
```

Checks live in the pre-commit hooks and the `justfile`, and nowhere else — CI runs those same entry points:

```bash
just check   # the full gate set
just test    # the test suite
just lint    # formatting and lints
```

Install the hooks once with `pre-commit install` inside the devShell; they run at commit and push. What this repository owes every artifact it ships is stated in [docs/reference/quality-gates.md](./docs/reference/quality-gates.md), which is binding on any change that adds one.

## Documentation

Everything about the product lives under [docs/](./docs/README.md), organised by reader need: guides for task sequences, reference for exact contracts and formats, explanation for the method and the design, decisions for why. The reference zone is the normative specification this build conforms to; `AGENTS.md` states how the two relate while the build is under way.

Start at [docs/explanation/overview.md](./docs/explanation/overview.md) to learn the method, or at [docs/reference/record/zone-layout.md](./docs/reference/record/zone-layout.md) to build against the specification.

## License

Licensed under either of [LICENSE-APACHE](./LICENSE-APACHE) (Apache License 2.0) or [LICENSE-MIT](./LICENSE-MIT) (MIT license) at your option.
