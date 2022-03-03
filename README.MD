# Minigrap

Its a simple terminal application ho receive 3 terminal inputs

- **Query**: this param set the word you want to find in the file.
- **Filename**: this param set the file that you want to read from.
- **case_sensitive**: this enviroment variable saids to the app if have to get case sensitive, the default value is **false**.

## How to use

You can run directly with cargo

```bash
cargo run to poem.txt
```

> **Note:** the **to** is the query and **poem.txt** is the filename

or you can build the app and execute the bin file, inside target > debug > minigrap

```bash
cargo build
```

```bash
./target/debug/minigrep to poem.txt
```

For using the **case_sensitive** variable

```bash
export CASE_INSENSITIVE=true
```

or

```bash
export CASE_INSENSITIVE=false
```
