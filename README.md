![GitHub Release](https://img.shields.io/github/v/release/toshiki670/yt-tool)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/toshiki670/yt-tool/rust.yml)

# yt tool

## Install from GitHub Releases

```
cargo install --git https://github.com/toshiki670/yt-tool.git --tag v0.4.0
```

## Install from local

```
cargo install --path .
```

## Release flow

1. Create a new release Pull Request
  * `makers release-major`
  * `makers release-minor`
  * `makers release-patch`
  * `makers release-rc`
  * `makers release-beta`
  * `makers release-alpha`
2. Merge and close the release pull request

## zsh completion

```
if [ -e ~/.zsh/completions ]; then
  fpath=(~/.zsh/completions $fpath)
fi
autoload -U compinit
compinit
```
