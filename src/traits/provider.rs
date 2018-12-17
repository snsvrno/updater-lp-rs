use failure::Error;

use version::Version;

pub trait Provider {
    fn valid_url(&self) -> bool;
    fn get_available_versions(&self) -> Result<Vec<Version>,Error>;
    fn get_latest_version(&self) -> Result<Version,Error>;
    fn get_link_for(&self, version : &Version) -> Result<String,Error>; 

    fn get_link_for_latest(&self) -> Result<(String,Version),Error> {

        let mut versions = self.get_available_versions()?;
        versions.sort();
        versions.reverse();
         
        for v in versions {
            if let Ok(link) = self.get_link_for(&v) {
                return Ok((link,v));
            }
        }

        Err(format_err!("No compatible versions found."))
    }
}