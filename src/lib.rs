#[macro_use] extern crate log;
#[macro_use] extern crate failure; use failure::Error;
#[macro_use] extern crate serde_derive;
extern crate version_lp as version; use version::Version;
extern crate regex;
extern crate restson;

extern crate download_lp as download;
extern crate archive_lp as archive;
extern crate platform_lp as platform;

use std::fs;
use std::env;

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

pub fn update_with_version(repo_link : &str, version : &Version) -> Result<(),Error> {
    let link = get_link_for_version(repo_link, version)?;
    println!("{}",link);

    let exe_path = env::current_exe().unwrap();

    let (file,size) = download::download(&link,".")?;

    match exe_path.file_name() {
        None => { return Err(format_err!("Can't determine executable name")); }
        Some(file_name) => {
            if archive::contains_file(&file, file_name.to_str().unwrap())? {
                match exe_path.parent() {
                    None => { return Err(format_err!("Can't determine executable path")); }
                    Some(parent) => {
                        fs::remove_file(&exe_path);
                        archive::extract_root_to(&file,&parent.display().to_string());
                        fs::remove_file(file);
                    }
                }
            }
        }
    }
    Ok(())
}