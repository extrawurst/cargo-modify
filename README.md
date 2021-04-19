# cargo-modify

currently a proof of concept for being able to switch the new resolver on via a cmd line call during a CI build job:

```sh
# remove 'resolver' from Cargo.toml
cargo modify --new-resolver=false
# set new resolver
cargo modify --new-resolver=true
# as above, this is the default
cargo modify
```

**Caution** this will reorder your `Cargo.toml` arbitrarily. Primary use-case is to do on-the-fly changes on a CI

later we could use this to allow arbitrary edits on a `Cargo.toml`