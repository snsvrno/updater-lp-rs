# updater-lp

A no frills updater that uses Github has its base. 
Easily create auto-updating for your cli apps!

## How it Works?

Host your code on Github! Tag your code with version releases and then
attach builds to those releases. 

_updater-lp_ then looks for the release at your repository and then version
matches those release against what you state your current app version is. If 
it finds a newer version, it can be used to 'update' your app by replacing 
your current binary with the new binary from Github.

## How to use it.

First off, a [test app](https://github.com/snsvrno/updater-lp-rs/tree/master/src-test)
is in the source repository that shows you how its used, as well as is used to test
the libraries functionality.

### Include the library

Add _updater-lp_ to your `cargo.toml`.

```toml
[dependencies]
updater-lp = "0.2"
```

### Using the Library

Then you need to set your upstream address. _updater-lp_ is struct-less (or better called static)
so its recommended you have a `&'STATIC str` with your repository address.

``` rust
static REPO_PATH : &str = "https://github.com/snsvrno/lpsettings-rs";
```

And then where ever it makes sense, you should check for the latest version. I'd recommend putting 
someting to prevent checking everytime the program runs (so you don't perform too many needless github
api calls) and instead check for updates daily.

```rust
let this_version = updater_lp::create_version(&[01,2,3,4]);

match updater_lp::get_latest_version(REPO_PATH) {
    Err(error) => { 
        // can't check the version for some reason, `error` should state why.
    },
    Ok(latest) => {
        if latest > this_version {
            updater_lp::update_with_version(REPO_PATH,&latest);
        }
    }
}
``` 

I'd recommend against using `?` on `get_latest_version` because you probably don't want your program
error / fail if you can't connect to the repo.

### A note of Versions

_updater-lp_ uses [version-lp](https://crates.io/crates/version-lp) for all versions. This means 
that versions are easily compairable. Version is exposed in this crate to allow for easy use
without requiring you to add an additional dependency to your `cargo.toml`