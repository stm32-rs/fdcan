# What to do to publish a new release

* Update Cargo.toml
* Update CHANGELOG.md
* Update version in README.md

```
git commit -am 'v0.2.0'
git push upstream

# Merge this commit onto the master branch

git fetch upstream
git checkout upstream/master
git tag -a 'v0.2.0' -m 'v0.2.0'
git push upstream v0.2.0
cargo publish --features fdcan_g0_g4_l5
```
