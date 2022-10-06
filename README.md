# Quicksearch

Configurable quick search engine shortcuts for your terminal.

## Installation

Run `cargo install quicksearch` to install

Run `quicksearch config` to get the path to your config.

## Usage

With this example set up:

```json
{
    "entries": {
        "yt": "https://www.youtube.com/results?search_query=%s"
    }
}
```

You can search YouTube with: `quicksearch search yt Never Gonna Give You Up`

Not quick enough? You can set an alias in your shell profile, eg. for zsh:

```sh
alias q="quicksearch search"
```

Then you can simply search: `q yt Never Gonna Give You Up`!

Still not quick enough? You can use the shell integration in your shell profile. eg. for zsh:

```sh
eval "$(quicksearch shell zsh)"
```

Then you can simply search `yt Never Gonna Give You Up`!
