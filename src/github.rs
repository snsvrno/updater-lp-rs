use version::version::Version;
use lperror;
use restson;
use ansi_term::Colour::{Blue};

#[derive(Serialize,Deserialize,Debug)]
struct Release {
    url: String,
    tag_name: String,
    tarball_url: String,
    zipball_url: String,
}

impl restson::RestPath<(String,String)> for Release {
    fn get_path(pair: (String,String)) -> Result<String,restson::Error> { Ok(format!("repos/{}/{}/releases/latest",pair.0,pair.1)) }
}

pub fn has_updates(version : &Version,username : &str, reponame : &str) -> Result<bool,lperror::LovepackError> {
  info!(target:"updater-lp","using {}",Blue.paint("Github"));

  if let Ok(mut client) = restson::RestClient::new("https://api.github.com") {
  //if let Ok(mut client) = restson::RestClient::new("http://httpbin.org") {
    let release : Result<Release,restson::Error> = client.get((username.to_string(),reponame.to_string()));
    
    if let Ok(release) = release {
      if let Some(latest_version) = Version::from_str(&release.tag_name) {
        println!("{} >= {} : {:?}",latest_version,version,latest_version >= version.clone());
      }
    }
    
  }


  return Ok(true);
}