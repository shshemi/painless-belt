# Painless Belt

Painless Belt (`pb`) is a lightweight CLI that wraps macOS's Seatbelt API to run executables inside a sandbox. It pairs a friendly command-line with a simple profile system and ships with ready-made profiles for common tools.

## Install

```sh
cargo install painless-belt
```

That gives you a binary called `pb`.

## Quick start

Run any command in the default sandbox:

```sh
pb -- ls
```

The `--` separates `pb`'s own options from the command that should be sandbox.
The default profile lets the command read and write into the current directory,
read system libraries, and reach the network — enough to be useful, not
enough to be scary.

Profiles are a starting point. Extra rules to allow/deny operations could be easily added on top via CLI options. For example, block a command from reaching the network:

```sh
pb --deny-network-outbound -- curl https://github.com
```

It is also possible to start from an empty profile and grant exactly what's needed:

```sh
pb --profile empty --allow-file-read-subpath /tmp -- cat /tmp/hello.txt
```

Every SBPL rule has a matching CLI flag. They all follow the same shape:

```
--{allow|deny}-<operation>[-<filter>] <value>
```

A few examples:

```sh
# Let a script read your Downloads folder and write to /tmp:
pb --allow-file-read-subpath ~/Downloads \
   --allow-file-write-subpath /tmp \
   -- ./my-script.sh

# Outbound HTTPS only:
pb --profile empty \
   --allow-system-binaries \
   --allow-network-outbound-remote tcp:*:443 \
   -- curl https://example.com

# Block everything under ~/.ssh even if the profile would allow it:
pb --deny-file-read-subpath ~/.ssh -- some-tool
```

There are also non-sbpl convenience shortcuts for common asks:

```sh
--allow-system-binaries     # run binaries from $PATH
--allow-system-libraries    # load /usr/lib, /System, /Library, ...
--allow-read-home           # read ~
--allow-write-home          # write ~
--allow-readwrite-home      # both
--allow-read-keychain       # read ~/Library/Keychains
```

Run `pb --help` to see the full list.

## Profiles

A profile is a reusable bundle of sandbox rules. They live in
`~/.painless-belt/profiles/<name>.pb` and use SBPL with a sprinkle of
Jinja templating (`{{ home }}`, `{{ pwd }}`, `{% include "other" %}`).

Use a profile with `-p` / `--profile`:

```sh
pb --profile python -- python script.py
pb -p node -- npm install
```

Two profiles come built-in:

- `default` — the sensible everyday profile (used when you don't pass `-p`)
- `empty` — a deny-everything baseline to build on

### Pulling profiles

A handful of curated profiles live in the [project repo](https://github.com/shshemi/painless-belt/tree/master/profiles).

Pull one to your machine:

```sh
pb pull claude
pb pull python
pb pull go
```

That saves the profile to `~/.painless-belt/profiles/<name>.pb`, ready to
use with `-p <name>`.

To remove a profile you pulled:

```sh
pb remove python
```

### Cloning and editing

To tweak a profile, clone it under a new name and edit:

```sh
pb clone python my-python      # works whether 'python' is local or upstream
pb edit my-python              # opens $EDITOR (or $VISUAL, or vi)
```

`pb clone` first tries to copy a local profile; if it's not there, it falls back to pulling from upstream

## Binding profiles to commands

You can assign profile to be used for commands, so you don't have to type `-p` every time.

Create `~/.painless-belt/config`:

```toml
[profile_map]
claude  = "claude"
python  = "python"
python3 = "python"
node    = "node"
npm     = "node"
git     = "gh"
```

`pb` looks up the first word of the command and picks the matching profile automatically:

```sh
pb -- claude          # uses the 'claude' profile
pb -- python script.py # uses the 'python' profile
pb -- node app.js     # uses the 'node' profile
pb -- something-else  # falls back to 'default'
```

You can still override on the fly:

```sh
pb -p empty -- python script.py
```

## License

MIT
