# Releasing

A release is driven entirely by pushing a `v*` tag. The
`Build Debian Package` workflow then:

1. Audits `Cargo.lock` against the RustSec advisory database.
2. Builds `.deb` packages for amd64 and arm64.
3. Creates a GitHub release with the `.deb` files attached.
4. Publishes the crate to crates.io via Trusted Publishing (no token is
   stored in this repository).
5. Triggers a rebuild of the APT repository (`charlieh0tel/apt-repo`).

Steps 3–5 only run if the audit and builds succeed, so a tag whose
dependencies carry an advisory never reaches crates.io or the APT repo.

## Steps

1. Start from an up-to-date, clean `main`:

   ```sh
   git switch main
   git pull --ff-only
   ```

2. Check for advisories, updating the lockfile if needed:

   ```sh
   cargo audit
   cargo update   # if cargo audit reports anything
   cargo audit
   ```

   Commit any lockfile change on its own.

3. Bump `version` in `Cargo.toml`, then rebuild so `Cargo.lock` picks it
   up:

   ```sh
   cargo build
   ```

4. Verify:

   ```sh
   cargo fmt --check
   cargo clippy --all-targets
   cargo test
   ```

5. Commit, tag and push:

   ```sh
   git commit -am "Bump version to X.Y.Z"
   git tag -a vX.Y.Z -m "vX.Y.Z"
   git push origin main
   git push origin vX.Y.Z
   ```

6. Watch the tag's workflow run and confirm the release:

   ```sh
   gh run watch $(gh run list --branch vX.Y.Z --limit 1 --json databaseId -q '.[0].databaseId')
   gh release view vX.Y.Z
   cargo search usbrelay-rs --limit 1
   ```

## Notes

- Never move or re-push a published tag. If a release fails after the tag
  is pushed, fix the problem and release the next patch version.
- crates.io versions are permanent; a version can be yanked but not
  replaced.
