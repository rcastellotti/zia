# Zia, visual testing for CLIs

> [!CAUTION]
> This project is in active devlopment, stuff will break, you have been warned bear :) .

# How to:

Install [astral-sh/uv](https://github.com/astral-sh/uv)

Create a `config.toml` file, example:

```toml
bin1 = "/bin/ls"
bin2 = "/opt/homebrew/bin/eza"
cmds = ["-1 | sort -n", "-la", "-ll"]
```

Run:

- `uv run main.py run --html`
- `uv run main.py serve`

## TODOs

- TODO: implement "scenarios" to bundle commands toghether
