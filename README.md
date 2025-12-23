# totp

An extremely fast TOTP account manager, written in Rust.

## Building

```bash
cargo build --profile release
```

## Features

```console
$ totp
An extremely fast TOTP account manager.

Usage: totp.exe <COMMAND>

Commands:
  list    List all saved accounts (use --secret to include secrets)
  path    Print the JSON file path
  add     Add a new account (ignored if it already exists)
  remove  Remove an account (requires confirmation)
  clean   Remove all accounts (requires confirmation)
  get     Show the current code and copy to clipboard (skip copy with -n | --no-copy)
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### list

Print out a list of all saved accounts. You can use the flag `--secret` to also include the secrets.

```console
$ totp list -h
List all saved accounts (use --secret to include secrets)

Usage: totp.exe list [OPTIONS]

Options:
      --secret  Include secrets
  -h, --help    Print help

$ totp list
╭───┬──────────────────╮
│   │ name             │
├───┼──────────────────┤
│ 0 │ GitHub.FooBar    │
│ 1 │ X-Y-ZUser123     │
│ 2 │ Modrinth.Voximir │
╰───┴──────────────────╯

$ totp list --secret
╭───┬──────────────────┬──────────────────────────────────╮
│   │ name             │ secret                           │
├───┼──────────────────┼──────────────────────────────────┤
│ 0 │ GitHub.FooBar    │ XXXXXXXXXXXXXXXX                 │
│ 1 │ GitHub.voximir-p │ YYYYYYYYYYYYYYYY                 │
│ 2 │ Modrinth.Voximir │ ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ │
╰───┴──────────────────┴──────────────────────────────────╯
```

### path

Print out the path to the JSON file.<br>
Note: Manually editing the JSON file is allowed but not encouraged.

```console
$ totp path -h
Print the JSON file path

Usage: totp.exe path

Options:
  -h, --help  Print help

$ totp path
C:\Users\FooBar\.totp\secrets.json
```

### add

Add a new account. You will be warned if it already exists.

```console
$ totp add -h
Add a new account (ignored if it already exists)

Usage: totp.exe add <NAME> <SECRET>

Arguments:
  <NAME>    The account's name
  <SECRET>  The account's secret

Options:
  -h, --help  Print help

$ totp add Bob XXXXXXXXXXXXXXXX
Account added: Bob

$ totp add Bob YYYYYYYYYYYYYYYY
Account: Bob already exists, please remove it first.
```

### remove

Ask the user whether they confirm the deletion. Also prints the secret when deleted to give the user a second chance if it was a mistake.

```console
$ totp remove -h
Remove an account (requires confirmation)

Usage: totp.exe remove <NAME>

Arguments:
  <NAME>  The account's name

Options:
  -h, --help  Print help

$ totp remove Bob
Do you wish to delete this account: Bob (yes/[no]): yes
Removed account: Bob
Secret: XXXXXXXXXXXXXXXX
```

### clean

Run `totp list --secret` then asks the user whether they confirm the deletion.

```console
$ totp clean -h
Remove all accounts (requires confirmation)

Usage: totp.exe clean

Options:
  -h, --help  Print help

$ totp clean
╭───┬──────┬──────────────────╮
│   │ name │ secret           │
├───┼──────┼──────────────────┤
│ 0 │ Alex │ XXXXXXXXXXXXXXXX │
│ 1 │ Bob  │ YYYYYYYYYYYYYYYY │
╰───┴──────┴──────────────────╯

Do you wish to delete all of these accounts? This action is irreversible. (yes/[no]): yes
Removed all accounts.
```

### get

Print out the current TOTP code for the selected account.<br> If no name was passed in, it will ask for the ID.<br>
Normally, the code will be automatically copied to the clipboard. You can disable this with the flag `-n` or `--no-copy`.

```console
$ totp get -h
Show the current code and copy to clipboard (skip copy with -n | --no-copy)

Usage: totp.exe get [OPTIONS] [NAME]

Arguments:
  [NAME]  The account's name

Options:
  -n, --no-copy  Disable copying to the clipboard
  -h, --help     Print help

$ totp get Bob -n
Bob: 112358
Expires in 1 second.

$ totp get
╭───┬──────╮
│   │ name │
├───┼──────┤
│ 0 │ Alex │
│ 1 │ Bob  │
╰───┴──────╯

Select account from ID: 0
Alex: 123456
Expires in 17 seconds.
```

## License

totp is licensed under the MIT license ([LICENSE](LICENSE) or <https://opensource.org/licenses/MIT>)
