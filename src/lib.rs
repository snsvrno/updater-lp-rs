extern crate lperror;
extern crate version; use version::version::Version;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
extern crate regex;
extern crate ansi_term;
extern crate restson;

mod structs; use structs::repo_breakdown::RepoBreakdown;
mod providers;

// https://(1)/(2)/(3)

pub fn has_updates(version : &Version,repo_url : &str) -> Result<Option<Version>,lperror::LovepackError> {
  let repo_info = RepoBreakdown::new(repo_url);

  if repo_info.is_valid() {
    match repo_info.provider {
      structs::providers::Providers::Github => { return providers::github::has_updates(&version,&repo_info); }
      structs::providers::Providers::None => {
        return Err(lperror::LovepackError::Error(format!("Could not find provider for {}",repo_url))); 
      }
    }
  } else {
    return Err(lperror::LovepackError::Error(format!("Could not parse {}",repo_url)));
  }

}

pub fn get_latest(repo_url : &str) -> Result<String,lperror::LovepackError> {
  let repo_info = RepoBreakdown::new(repo_url);

  if repo_info.is_valid() {
    return match repo_info.provider {
      structs::providers::Providers::Github => providers::github::get_latest(&repo_info),
      structs::providers::Providers::None => Err(lperror::LovepackError::Error(format!("Could not get latest version's because no valid provider")))
    };
  } else {
    return Err(lperror::LovepackError::Error(format!("Error parsing {}",repo_url)));
  }
}