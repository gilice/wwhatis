
# wwhatis

`wwhatis` (Wiki, what is?) is a simple command line program that displays short summaries of concepts from [Wikipedia, the Free Encyclopedia](https://www.wikipedia.org). It's designed to help you understand a topic in an instant, while enabling you to conduct further research easily. 

> Internally, the project is named `wwhatis-rs` because this project started out as a [Dart](https://dart.dev) version, but I switched to [Rust](rust-lang.org/) for better speed (and for learning)
 

## Installation

Clone this respository, then install it using `cargo`.
```console
  $ git clone https://github.com/riceicetea/wwhatis && cd wwhatis
  $ cargo install --path .
```

or, on [NixOS](https://nixos.org), install it by adding the following to your `configuration.nix`:
```nix
let
    wwhatis_drv = fetchFromGitHub {
	    owner = "riceicetea";
	    repo = "wwhatis";
	    rev = "main";
        # fill out the sha256hash from here
	    sha256 = "...";
    };

    wwhatis_pkg = (import wwhatis_drv system).default;
in
# ...
{
    environment.systemPackages = with pkgs; [
    # ...
    wwhatis_pkg
    # ...
    ];
};
```

## Usage/Examples

```console
wwhatis-rs 0.1.0
Quickly displays summaries of given topics. This program displays text from Wikipedia, the Free
Encyclopedia. Check it out at https://wikipedia.org

USAGE:
    wwhatis [OPTIONS] [TOPICS]...

ARGS:
    <TOPICS>...    

OPTIONS:
    -a, --about          Print the about page and info about used libraries
    -f, --formatless     Enable if running in a terminal that doesn't support ANSI escape codes
    -h, --help           Print help information
    -l, --lang <LANG>    The language prefix of the wiki's url that you want to use, like "en" or
                         "de" [default: en]
    -m, --mobile         Return URL in mobile (m.wikipedia.org) version
    -o, --open           Open the article URLs on the last step in your default web browser with
                         xdg-open (only works on *nix)
    -V, --version        Print version information
```


## Building a release version
 - generate about page (it should be auto-generated for you on every commit, but to be sure do it again) with [cargo-about](https://crates.io/crates/cargo-about)
 - `nix build`

### Optional: create release bundle
 - compress the built binary with [upx](https://upx.github.io)
 - creates a `tar.gz` with the documentation and binary


## Acknowledgements
Check `Cargo.toml` for the full list of packages.

- **Programming anguage:** [Rust](https://rust-lang.org)

- **HTTP client, parallelism, parsing**: [hyper](https://hyper.rs/) and [hyper-tls](https://crates.io/crates/hyper-tls), [tokio](https://github.com/tokio-rs/tokio) and [futures](https://crates.io/crates/futures), [serde](https://serde.rs)

- **Terminal features:** [crossterm](https://crates.io/crates/crossterm), [dont_disappear](https://crates.io/crates/dont_disappear), [spinners](https://crates.io/crates/spinners), [clap](https://crates.io/crates/clap) 
- [readme.so](https://readme.so), online README *ide*
- [How to write a Good README](https://bulldogjob.com/news/449-how-to-write-a-good-readme-for-your-github-project)
