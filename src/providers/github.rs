use version::version::Version;
use lperror;
use restson;
use ansi_term::Colour::{Blue,Green,Yellow,Red};

use structs::repo_breakdown::RepoBreakdown;

#[derive(Serialize,Deserialize,Debug)]
struct Release {
    url: String,
    tag_name: String,
    tarball_url: String,
    zipball_url: String,
}

impl<'a> restson::RestPath<&'a RepoBreakdown> for Release {
    fn get_path(repo: &'a RepoBreakdown) -> Result<String,restson::Error> { Ok(format!("repos/{}/{}/releases/latest",repo.username,repo.repo)) }
}

pub fn has_updates(version : &Version,repo_info : &RepoBreakdown) -> Result<Option<Version>,lperror::LovepackError> {
  //! checks if there is a newer version available.
  info!(target: "updater-lp-rs", "Checking for update using {}",Blue.paint("Github"));

  match restson::RestClient::new("https://api.github.com") {
    Err(error) => { return Err(lperror::LovepackError::Error(restson_error_to_string(error))); },
    Ok(mut client) => {
      let release : Result<Release,restson::Error> = client.get(repo_info);
      match release {
        Err(error) => return Err(lperror::LovepackError::Error(restson_error_to_string(error))),
        Ok(release) => {
          if let Some(latest_version) = Version::from_str(&release.tag_name) {
            info!(target: "updater-lp-rs", "Compairing local version of {} against latest version of {}",Yellow.paint(version.to_string()),Yellow.paint(latest_version.to_string()));
            if latest_version > version.clone() {
              return Ok(Some(latest_version.clone()));
            }
          }
        }
      }
    }
  }
  Ok(None) // version isn't newer.
}

pub fn get_latest(repo_info : &RepoBreakdown) -> Result<String,lperror::LovepackError> {
  //! gets the latest version's path

  match restson::RestClient::new("https://api.github.com"){
    Err(error) => { return Err(lperror::LovepackError::Error(restson_error_to_string(error))); }
    Ok(mut client) => {
      let release : Result<Release,restson::Error> = client.get(repo_info);
      return match release {
        Err(error) => Err(lperror::LovepackError::Error(restson_error_to_string(error))),
        Ok(release) => Ok(release.zipball_url)
      }
    }
  }

}

fn restson_error_to_string(error : restson::Error) -> String {
  //! dumb match function to get strings from the restson error codes.

  match error {
    restson::Error::HttpClientError => format!("{} HTTP Client {}", Green.paint("Reston"),Red.paint("Error")),
    restson::Error::UrlError => format!("{} URL {}", Green.paint("Reston"),Red.paint("Error")),
    restson::Error::ParseError => format!("{} Parse {}", Green.paint("Reston"),Red.paint("Error")),
    restson::Error::RequestError => format!("{} Request {}", Green.paint("Reston"),Red.paint("Error")),
    restson::Error::TimeoutError => format!("{} Timeout {}", Green.paint("Reston"),Red.paint("Error")),
    restson::Error::HttpError(unum,string) => format!("{} HTTP {} {}: {}",Green.paint("Reston"),unum,Red.paint("Error"),string),
  }
}