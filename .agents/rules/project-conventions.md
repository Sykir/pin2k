# Pin2k Project Conventions

1. **Active Repository**: Always work in `/workspace/pin2k` (persistent Kubernetes PVC).
2. **English-Only Documentation**: All Markdown documentation (`README.md`, `docs/*.md`) must be written in English, regardless of the conversation language.
3. **Safety**: Keep `pin2k-core` 100% safe Rust (`zero unsafe`).
4. **Secret Hygiene**: Never write tokens, PATs, or credentials into `.git/config`, tracked files, or responses. When pushing to an external remote requiring a token, pass it transiently via `git -c http.<url>.extraheader=...` only when requested.
