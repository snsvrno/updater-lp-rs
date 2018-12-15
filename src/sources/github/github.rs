use failure::Error;
use regex::Regex;
use restson::RestClient;

use version::Version;
use platform::Platform;

use traits::provider::Provider;

use sources::github:: {
    params:: { GithubParams, REPO_URL, API, NAME },
    restson:: GitRest,
};

pub struct Github {
    repo_link : String
}

impl Provider for Github {
    
    fn valid_url(&self) -> bool {
        let re = Regex::new(REPO_URL).unwrap();
        re.is_match(&self.repo_link)
    }
    
    fn get_latest_version(&self) -> Result<Version,Error> {
        let re = Regex::new(REPO_URL).unwrap();
        if let Some(captures) = re.captures(&self.repo_link) {
            let params = GithubParams {
                username : captures.get(1).unwrap().as_str().to_string(),
                repo : captures.get(2).unwrap().as_str().to_string(),
            };

            let mut client =  RestClient::new(API)?;
            let release : GitRest = client.get(&params)?;
            
            match Version::from_str(&release.tag_name){
                None => return Err(format_err!("Error parsing version {}",release.tag_name)),
                Some(version) => return Ok(version),
            }
        }

        Err(format_err!("No versions found"))
    }

    fn get_available_versions(&self) -> Result<Vec<Version>,Error> {
        let mut versions : Vec<Version> = Vec::new();

        let re = Regex::new(REPO_URL).unwrap();
        if let Some(captures) = re.captures(&self.repo_link) {
            let params = GithubParams {
                username : captures.get(1).unwrap().as_str().to_string(),
                repo : captures.get(2).unwrap().as_str().to_string(),
            };

            let mut client =  RestClient::new(API)?;
            let release : Vec<GitRest> = client.get(&params)?;

            for r in release {
                match Version::from_str(&r.tag_name){
                    None => { warn!("Error parsing version {}",r.tag_name) },
                    Some(version) => { versions.push(version)},
                }
            }
            
        }

        Ok(versions)
    }

    fn get_link_for(&self, version : &Version) -> Result<String,Error> {

        let re = Regex::new(REPO_URL).unwrap();
        if let Some(captures) = re.captures(&self.repo_link) {
            let params = GithubParams {
                username : captures.get(1).unwrap().as_str().to_string(),
                repo : captures.get(2).unwrap().as_str().to_string(),
            };

            let mut client =  RestClient::new(API)?;
            let release : Vec<GitRest> = client.get(&params)?;

            for r in release {
                match Version::from_str(&r.tag_name){
                    None => { warn!("Error parsing version {}",r.tag_name) },
                    Some(v) => { 
                        if &v == version {
                            let platform = Platform::get_user_platform();
                            for asset in r.assets {
                                if platform == Platform::new(&asset.browser_download_url) {
                                    return Ok(asset.browser_download_url);
                                }
                            }
                        }
                    },
                }
            }
            
        }
        Err(format_err!("No release found for version {}",version))
    }
}
