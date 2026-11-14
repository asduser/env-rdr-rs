# env-rdr-rs

Native implementation w/o any packages. 🔥

Loads values from the process environment,
falling back to a `.env` file in the current directory. No dependencies.

## Run 💻

```bash
    cp .env.example .env    # then edit it
    chmod 600 .env
    cargo run
```

Environment variables override the file:

```bash
    MY_URL=other.example.com cargo run
```

## Test ☕

```bash
    cargo test
```

## .env format 📄

- `KEY=value`, one per line; `export KEY=value` also works
- Blank lines and lines starting with `#` are ignored
- One matching pair of surrounding `"` or `'` quotes is stripped
- `$` is literal (no variable substitution), so values like bcrypt hashes are safe
- If a key appears more than once, the last one wins

The password is redacted in `Debug` output, so `{:?}` never prints it.
