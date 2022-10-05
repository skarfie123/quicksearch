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

Not quick enough? You can set an alias in your shell config, eg. for zsh:

```sh
alias q="quicksearch search"
```

Then you can simply search: `q yt Never Gonna Give You Up`!
