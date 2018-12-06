use version::Version;
use failure::Error;

pub trait Provider {
    fn valid_url(repo_url : &str) -> bool;
    
    fn get_available_versions(repo_url : &str) -> Result<Vec<Version>,Error>;
    fn get_latest_version(repo_url : &str) -> Result<Version,Error>;
    fn get_link_for(repo_url : &str, version : &Version) -> Result<String,Error>; 
}