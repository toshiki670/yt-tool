![GitHub Release](https://img.shields.io/github/v/release/toshiki670/yt-tool)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/toshiki670/yt-tool/rust_check.yml)

# yt tool

## Release flow

1. Create a release branch as `release/v[0-9]+.[0-9]+.[0-9]+`
2. Update the version at `[workspace.package].version` in each `Cargo.toml`
3. Push the release branch
4. Create the release pull request
5. Change the name of the release pull request to `v[0-9]+.[0-9]+.[0-9]+`
6. Merge and close the release pull request
7. Verify that it is released

## zsh completion

```
if [ -e ~/.zsh/completions ]; then
  fpath=(~/.zsh/completions $fpath)
fi
autoload -U compinit
compinit
```
