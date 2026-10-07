---
status: superseded by ADR 0005
---

# Man-page help layout instead of clap's default

Help pages were to use a man-page layout (NAME, SYNOPSIS, DESCRIPTION, … EXAMPLES) rendered by our own code, with clap's `--help` overridden to show the same page. This was chosen for its familiarity to Unix users. It was reversed before any code was written; see [ADR 0005](0005-clap-style-help.md).
