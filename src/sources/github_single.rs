use Provider;

use failure::Error;
use regex::Regex;
use restson::RestClient;

use version::Version;
use platform::Platform;

static REPO_URL : &str = r"https://github.com/([^/]*)/([^/]*)";
static API : &str = r"https://api.github.com";
static NAME : &str = "Github";

struct GithubParams {
  username: String,
  repo : String,
}


#[derive(Serialize,Deserialize,Debug)]
pub struct Github { 
    url: String,
    tag_name: String,
    assets: Vec<Asset>
}

#[derive(Serialize,Deserialize,Debug)]
struct Asset {
    name: String,
    browser_download_url: String,
}

// for getting the latest one
impl<'a> restson::RestPath<&'a GithubParams> for Github {
    fn get_path(repo: &'a GithubParams) -> Result<String,restson::Error> { 
        Ok(format!("repos/{}/{}/releases/latest",repo.username,repo.repo)) 
    }
}

// for getting all of them
impl<'a> restson::RestPath<&'a GithubParams> for Vec<Github> {
    fn get_path(repo: &'a GithubParams) -> Result<String,restson::Error> { 
        Ok(format!("repos/{}/{}/releases",repo.username,repo.repo)) 
    }
}

impl Provider for Github {
    
    fn valid_url(repo_url : &str) -> bool {
        let re = Regex::new(REPO_URL).unwrap();
        re.is_match(repo_url)
    }
    
    fn get_latest_version(repo_url : &str) -> Result<Version,Error> {

        let re = Regex::new(REPO_URL).unwrap();
        if let Some(captures) = re.captures(repo_url) {
            let params = GithubParams {
                username : captures.get(1).unwrap().as_str().to_string(),
                repo : captures.get(2).unwrap().as_str().to_string(),
            };

            let mut client =  RestClient::new(API)?;
            let release : Github = client.get(&params)?;
            
            match Version::from_str(&release.tag_name){
                None => return Err(format_err!("Error parsing version {}",release.tag_name)),
                Some(version) => return Ok(version),
            }
        }

        Err(format_err!("No versions found"))
    }

    fn get_available_versions(repo_url : &str) -> Result<Vec<Version>,Error> {
        let mut versions : Vec<Version> = Vec::new();

        let re = Regex::new(REPO_URL).unwrap();
        if let Some(captures) = re.captures(repo_url) {
            let params = GithubParams {
                username : captures.get(1).unwrap().as_str().to_string(),
                repo : captures.get(2).unwrap().as_str().to_string(),
            };

            let mut client =  RestClient::new(API)?;
            let release : Vec<Github> = client.get(&params)?;

            for r in release {
                match Version::from_str(&r.tag_name){
                    None => { warn!("Error parsing version {}",r.tag_name) },
                    Some(version) => { versions.push(version)},
                }
            }
            
        }

        Ok(versions)
    }

    fn get_link_for(repo_url : &str, version : &Version) -> Result<String,Error> {

        let re = Regex::new(REPO_URL).unwrap();
        if let Some(captures) = re.captures(repo_url) {
            let params = GithubParams {
                username : captures.get(1).unwrap().as_str().to_string(),
                repo : captures.get(2).unwrap().as_str().to_string(),
            };

            let mut client =  RestClient::new(API)?;
            let release : Vec<Github> = client.get(&params)?;

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

#[cfg(test)]
mod test {
    use traits::providers::Provider;
    use providers::github::Github;
    use version::Version;
    
    #[test]
    fn valid_link() {
        assert!(Github::valid_url("https://github.com/snsvrno/lpsettings-rs"));
        assert!(!Github::valid_url("https://gitlab.com/snsvrno/lpsettings-rs"));
        assert!(!Github::valid_url("https://www.google.com"));
    }

    #[test]
    #[ignore]
    fn get_latest_version() {
        let result = Github::get_latest_version("https://gitlab.com/snsvrno/lpsettings-rs");
        assert!(result.is_err());

        let result = Github::get_latest_version("https://github.com/snsvrno/lpsettings-rs");
        assert_eq!(result.unwrap(),Version::new(&[0,1,7]));
    }

    #[test]
    #[ignore]
    fn get_available_versions() {
        let result = Github::get_available_versions("https://gitlab.com/snsvrno/lpsettings-rs");
        assert!(result.is_err());

        let result = Github::get_available_versions("https://github.com/snsvrno/lpsettings-rs");
        println!("{:?}",result);
        assert!(false);
    }

    #[test]
    #[ignore]
    fn get_link_for() {
        let result = Github::get_link_for("https://gitlab.com/snsvrno/lpsettings-rs",&Version::new(&[2,0]));
        assert!(result.is_err());

        let result = Github::get_link_for("https://github.com/snsvrno/lpsettings-rs",&Version::new(&[0,1,7]));
        println!("{:?}",result);

        let result = Github::get_link_for("https://github.com/snsvrno/lpsettings-rs",&Version::new(&[0,1,6]));
        println!("{:?}",result);
        assert!(false);
    }
}
