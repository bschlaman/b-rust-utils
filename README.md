## Intro
This package is a toolbox of cli driven utilities written in rust.

The utilities aim to have a minimal interface and are intended for use in modern terminal emulators.

## Utilities

### `btime`

Displays local and utc date + time information in several useful formats.

```console
+------------------------------------------------------------------------------+
| Description            Local                    UTC                   Length |
+==============================================================================+
| Package name       📦  b-rust-utils                                          |
| Unix               ⏰️  1694034020                                     10     |
|                                                                              |
| Friendly Calendar  📅  Wed Sep  6 21:00 +00:00  Wed Sep  6 21:00 UTC  20     |
| RFC 3339           ⌛  2023-09-06T21:00:20Z     2023-09-06T21:00:20Z  20     |
| ISO 8601           📄  2023-09-06 21:00         2023-09-06 21:00      16     |
| ISO 8601 Date      💻  2023.09.06               2023.09.06            10     |
+------------------------------------------------------------------------------+
```

### `request`

**BETA FEATURE**

Essentially a `curl` clone that displays various information about the request.

Example:
```bash
request https://www.schlamalama.com
```

