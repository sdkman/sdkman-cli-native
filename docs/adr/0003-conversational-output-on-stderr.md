# Conversational output goes to stderr

Only data a script might capture (paths, versions and tables) goes to stdout. Everything conversational goes to stderr, including success messages such as `✓ Installed java 21.0.2 (Temurin)`, as well as warnings, errors, hints and prompts. This keeps `$(sdk home java 21.0.2-tem)` and similar captures clean, and follows cargo and gh. A success message on stderr is deliberate, not a bug. See [STYLE.md](../STYLE.md#streams).
