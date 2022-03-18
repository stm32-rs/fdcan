# What to do to publish a new release

* Update Cargo.toml
* Update CHANGELOG.md

```
git commit -am 'v0.2.0'
git push origin
git tag -a 'v0.2.0' -m 'v0.2.0'
git push origin v0.2.0
cargo publish --features fdcan_g0_g4_l5
```
