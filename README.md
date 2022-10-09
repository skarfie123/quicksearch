# Quicksearch

Configurable quick search engine shortcuts for your terminal and browser.

[![Crates.io](https://img.shields.io/crates/v/quicksearch?style=flat-square)](https://crates.io/crates/quicksearch)
[![Crates.io](https://img.shields.io/crates/d/quicksearch?style=flat-square)](https://crates.io/crates/quicksearch)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

## Installation

Run `cargo install quicksearch` to install

## Configuration

Run `quicksearch config` to get the path to your config.

Here is an example:

```json
{
    "entries": {
        "google": "https://www.google.com/search?q=%s",
        "yt": "https://www.youtube.com/results?search_query=%s",
        "ddg": "https://duckduckgo.com/?q=%s"
    },
    "default_engine": "ddg"
}
```

`engines` are required. This is a map between keywords and the search urls. `%s` is what will be replaced with your query.

`default_engine` is an optional setting for server mode that can be set to one of your engine keywords. If it is not set, unknown keywords will redirect to the quicksearch help page. With it set, your full query will be redirected to your default search engine. This allows you to use quicksearch as your default search engine in your browser.

## Usage

### Terminal

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

### Browser

Run the server with `quicksearch serve`. By default the port is `7878`, but you can set it with `--port [PORT]`.

Add quicksearch as a search engine, and set a keyword eg. `q` to use it easily. For Firefox see [here](https://support.mozilla.org/en-US/kb/add-or-remove-search-engine-firefox). For Chrome see [here](https://support.google.com/chrome/answer/95426) and set the url to `http://localhost:7878/%s` replacing `7878` with your chosen port.

At this point you can search YouTube with `q yt Never Gonna Give You Up`.

If you set quicksearch to be your default search engine, then you can simply search `yt Never Gonna Give You Up`. But any unmatched query will direct to the quicksearch help page. It is recommended to set `default_engine` in your config, so that any unmatched queries will fall through to your default search engine.

## Autostart Server

### Linux / MacOS

One option is to add to your crontab:

Run `crontab -e` to edit, then add `@reboot ~/.cargo/bin/quicksearch serve`.

### Windows

Open the startup folder (`Win+R` then run `shell:startup`).
In this folder, add a shortcut with `pwsh -windowstyle hidden -c quicksearch serve` as the location. You may need to replace `pwsh` with `powershell` depending on your version.

Alternatively, you could use Task Scheduler.
