# cargo-modify

Currently a proof of concept for being able to switch the new resolver on/off via a cmd line call during a **CI** build job.

## usage

run `cargo modify` in a folder that contains a `Cargo.toml`:

```sh
# set new resolver
cargo modify new-resolver
# same as above
cargo modify new-resolver --v2=true
# remove 'resolver' from Cargo.toml
cargo modify new-resolver --v2=false
```

**Caution** this will reorder your `Cargo.toml` arbitrarily. Primary use-case is to do on-the-fly changes on a CI

## more

later we could use this to allow arbitrary edits on a `Cargo.toml`, ideas welcome.