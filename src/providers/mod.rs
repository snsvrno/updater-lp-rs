use Provider;
mod github;

use version::Version;
use failure::Error;

pub enum List {
    Github(String),
    None(String)
}

impl List {
    pub fn get_provider(repo_url : &str) -> List {
        if github::Github::valid_url(repo_url) { return List::Github(repo_url.to_string()); }
        List::None(repo_url.to_string())
    }

    pub fn get_latest_version(&self) -> Result<Version, Error> {
        match self {
            List::None(ref link) => Err(format_err!("{} not a valid repo",link)),
            List::Github(ref link) => github::Github::get_latest_version(link)
        }
    }

    pub fn get_link_for_version(&self, version : &Version) -> Result<String,Error> {
        match self {
            List::None(ref link) => Err(format_err!("{} not a valid repo",link)),
            List::Github(ref link) => github::Github::get_link_for(link, &version),
        }
    }

    pub fn get_link_for_latest(&self) -> Result<String,Error> {
        match self {
            List::None(ref link) => Err(format_err!("{} not a valid repo",link)),
            List::Github(ref link) => {
                let version = github::Github::get_latest_version(link)?;
                github::Github::get_link_for(link, &version)
            }
        }
    }
}
