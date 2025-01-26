# yt tool

## Release flow

1. Create a release branch as `release/v[0-9]+.[0-9]+.[0-9]+`
2. Update the version at `[package].version` in each `Cargo.toml`
3. Push the release branch
4. Create the release pull request
5. Change the name of the release pull request to `v[0-9]+.[0-9]+.[0-9]+`
6. Merge and close the release pull request
7. Verify that it is released
