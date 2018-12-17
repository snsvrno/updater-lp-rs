//! # updater-lp
//! 
//! A no frills updater that uses Github has its base. 
//! Easily create auto-updating for your cli apps!
//! 
//! ## How it Works?
//! 
//! Host your code on Github! Tag your code with version releases and then
//! attach builds to those releases. 
//! 
//! _updater-lp_ then looks for the release at your repository and then version
//! matches those release against what you state your current app version is. If 
//! it finds a newer version, it can be used to 'update' your app by replacing 
//! your current binary with the new binary from Github.
//! 
//! ## How to use it.
//! 
//! First off, a [test app](https://github.com/snsvrno/updater-lp-rs/tree/master/src-test)
//! is in the source repository that shows you how its used, as well as is used to test
//! the libraries functionality.
//! 
//! ### Include the library
//! 
//! Add _updater-lp_ to your `cargo.toml`.
//! 
//! ```toml
//! [dependencies]
//! updater-lp = "0.2"
//! ```
//! 
//! ### Using the Library
//! 
//! Then you need to set your upstream address. _updater-lp_ is struct-less (or better called static)
//! so its recommended you have a `&'STATIC str` with your repository address.
//! 
//! ``` rust
//! static REPO_PATH : &str = "https://github.com/snsvrno/lpsettings-rs";
//! ```
//! 
//! And then where ever it makes sense, you should check for the latest version. I'd recommend putting 
//! someting to prevent checking everytime the program runs (so you don't perform too many needless github
//! api calls) and instead check for updates daily.
//! 
//! ```rust
//! # extern crate updater_lp;
//! # static REPO_PATH : &str = "https://github.com/snsvrno/lpsettings-rs";
//! let this_version = updater_lp::create_version(&[01,2,3,4]);
//! 
//! match updater_lp::get_latest_version(REPO_PATH) {
//!     Err(error) => { 
//!         // can't check the version for some reason, `error` should state why.
//!     },
//!     Ok(latest) => {
//!         if latest > this_version {
//!             updater_lp::update_with_version(REPO_PATH,&latest);
//!         }
//!     }
//! }
//! ``` 
//! 
//! I'd recommend against using `?` on `get_latest_version` because you probably don't want your program
//! error / fail if you can't connect to the repo.
//! 
//! ### A note of Versions
//! 
//! _updater-lp_ uses [version-lp](https://crates.io/crates/version-lp) for all versions. This means 
//! that versions are easily compairable. Version is exposed in this crate to allow for easy use
//! without requiring you to add an additional dependency to your `cargo.toml`

#[macro_use] extern crate log;
#[macro_use] extern crate failure; use failure::Error;
#[macro_use] extern crate serde_derive;
extern crate regex;
extern crate restson;

extern crate download_lp as download;
extern crate archive_lp as archive;
extern crate platform_lp as platform;
extern crate version_lp as version; use version::Version;

use std::fs;
use std::env;

mod sources;
mod source;
mod traits; use traits::provider::Provider;
// need to figure out how to register providers before i make this public
// pub use traits::providers::Provider;

pub fn get_latest_version(repo_link : &str) -> Result<Version,Error> {
    //! Checks the given Repository for the latest version
    
    let provider = source::get_provider(repo_link)?;
    provider.get_latest_version()
    
}

pub fn get_link_for_version(repo_link : &str, version : &Version) -> Result<String,Error> {
    //! Checks the given Repository for the compatible release of the given version. 
    //! 
    //! If there isn't any release for that version then it will return an Error.

    let provider = source::get_provider(repo_link)?;
    provider.get_link_for(version)
}

pub fn get_link_for_latest(repo_link : &str) -> Result<(String,Version),Error> {
    //! Checks the given Repository for the latest compatible release.
    //! 
    //! Might not necessarily be the latest version but rather is the
    //! latest version that has a release for the user's platform. Will
    //! return an error if there is connectivity issues or no releases
    //! were found for the user's platform.

    let provider = source::get_provider(repo_link)?;
    provider.get_link_for_latest()
}

pub fn create_version_raw(version_string : &[u8]) -> Version {
    //! A passthrough for the _version-lp_ crate, so you can create a version to compare against.
    //! 
    //! Accepts a reference to an Array of `u8` of any length.

    Version::new(version_string)
}

pub fn create_version(version_string : &str) -> Option<Version> {
    //! A passthrough for the _version-lp_ crate, so you can create a version to compare against.
    //! 
    //! Will attempt to create a version from a string.

    Version::from_str(version_string)
}

pub fn update_from_link(link : &str) -> Result<(),Error> {
    //! Updates the current application from the provided link.
    //! 
    //! It will validate the download and check that it contains an 
    //! executable that is named the same as this program otherwise 
    //! it will fail. This is to prevent the application from deleting
    //! itself without replacing itself with a proper replacement.
    //! 
    //! It will not check for platform compatibility. It is assuming that 
    //! already done. Designed to work with
    //! [get_link_from_latest()](fn.get_link_for_latest.html#func.get_link_for_latest)
    
    let exe_path = env::current_exe().unwrap();

    let (file,_) = download::download(&link,".")?;

    match exe_path.file_name() {
        None => { return Err(format_err!("Can't determine executable name")); }
        Some(file_name) => {
            if archive::contains_file(&file, file_name.to_str().unwrap())? {
                match exe_path.parent() {
                    None => { return Err(format_err!("Can't determine executable path")); }
                    Some(parent) => {
                        fs::remove_file(&exe_path)?;
                        archive::extract_root_to(&file,&parent.display().to_string())?;
                        fs::remove_file(file)?;
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn update_with_version(repo_link : &str, version : &Version) -> Result<(),Error> {
    //! Update the application with the specific version from the Repository.
    //! 
    //! Will fail if it cannot find an appropriate version / compatible platform.
    
    let link = get_link_for_version(repo_link, version)?;
    
    update_from_link(&link)
}