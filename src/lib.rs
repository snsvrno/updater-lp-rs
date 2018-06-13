#[macro_use]
extern crate output;
extern crate lperror;
extern crate version; use version::version::Version;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
extern crate regex;
extern crate ansi_term;
extern crate restson;

mod repo_breakdown; use repo_breakdown::RepoBreakdown;
mod github;
mod providers;

// https://(1)/(2)/(3)

pub fn has_updates(version : &Version,repo_url : &str) -> Result<bool,lperror::LovepackError> {
  let repo_info = RepoBreakdown::new(repo_url);

  if repo_info.is_valid() {
    match repo_info.provider {
      providers::Providers::Github => { return github::has_updates(&version,&repo_info); }
      providers::Providers::None => {
        return Err(lperror::LovepackError::Error(format!("Couldn't find provider for {}",repo_url))); 
      }
    }
  } else {
    return Err(lperror::LovepackError::Error(format!("error parsing {}",repo_url)));
  }

}

pub fn get_latest(repo_url : &str) -> Result<String,lperror::LovepackError> {
  let repo_info = RepoBreakdown::new(repo_url);

  if repo_info.is_valid() {
    return match repo_info.provider {
      providers::Providers::Github => github::get_latest(&repo_info),
      providers::Providers::None => Err(lperror::LovepackError::Error(format!("Couldn't get latest version's because no valid provider")))
    };
  } else {
    return Err(lperror::LovepackError::Error(format!("error parsing {}",repo_url)));
  }
}