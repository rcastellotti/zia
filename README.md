# Zia, e2e testing for CLIs

> [!CAUTION]
> This project is in active devlopment, stuff will break, you have been warned.  
 ʕ•ᴥ•ʔ.


## Yes but WHY?????

Why not? 

# Configuration

Zia needs a configuration file to know which commands it should run.

## `bin1`, `bin2`

Use these fields to set absolute path for the binaries you want to compare.

## `external_files`


If your tool modifies external files, you can include them in the (optional) `external_files` array. When this array is provided, the files are compared byte-by-byte, and any differences are reported.

> [!NOTE]
> Zia runs the commands sequentially and zeroes external files before running any command.

You can version control this file toghether with your source code.

## `[[commands]]`

The `[[commands]]` table is used to set commands to run, a sample entry looks like:

```toml
[[commands]]
name = "add Giorgio"
env = { IS_ENV_REQUIRED = "no" } # optional
args = ["add", "Giorgio", "Scuttlebuff"]
```

A few remarks:

+ `args` are used as arguments for [`.args()`](https://doc.rust-lang.org/std/process/struct.Command.html#method.args)
+ the `env` field is used to set environment variables, it is optional

# CLI Reference


## `--config <filename.toml>`

Use `--config <filename.toml>` option to choose a [configuration file](#configuration) to use.


P.S.: This is my first Rust project, help me make it better!