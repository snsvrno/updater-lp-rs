#[macro_use] extern crate log;
#[macro_use] extern crate failure; use failure::Error;
#[macro_use] extern crate serde_derive;
extern crate version_lp as version; use version::Version;
extern crate regex;
extern crate restson;

mod providers; use providers::List;
mod traits; 
pub use traits::providers::Provider;

pub fn get_latest_version(repo_link : &str) -> Result<Version,Error> {
    List::get_provider(repo_link).get_latest_version()
}

pub fn get_link_for_version(repo_link : &str, version : &Version) -> Result<String,Error> {
    List::get_provider(repo_link).get_link_for_version(&version)
}

pub fn get_link_for_latest(repo_link : &str) -> Result<String,Error> {
    List::get_provider(repo_link).get_link_for_latest()
}