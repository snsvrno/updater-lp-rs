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

mod github;
mod providers;

// https://(1)/(2)/(3)
static REPO_URL : &str = r"https://([^/]*)/([^/]*)/([^/]*)";

pub fn has_updates(version : &Version,repo_url : &str) -> Result<bool,lperror::LovepackError> {
  let re = regex::Regex::new(REPO_URL).unwrap();

  let mut provider = providers::Providers::None;
  let mut username : Option<String> = None;
  let mut repository_name : Option<String> = None;

  // checks the captures
  if let Some(captures) = re.captures(repo_url) {

    // checks for the provider
    if let Some(provider_string) = captures.get(1) {
      provider = match provider_string.as_str() {
        "github.com" => providers::Providers::Github,
        _ => providers::Providers::None,
      }
    }

    // checks for the username
    if let Some(grepped_username) = captures.get(2) {
      username = Some(grepped_username.as_str().to_string());
    }

    // checks for the reponame
    if let Some(grepped_reponame) = captures.get(3) {
      repository_name = Some(grepped_reponame.as_str().to_string());
    }
  }
  
  if !username.is_some() && !repository_name.is_some() {
    return Err(lperror::LovepackError::Error(format!("Couldn't parse url: {}",repo_url))); 
  }

  match provider {
    providers::Providers::Github => { return github::has_updates(&version,&username.unwrap(),&repository_name.unwrap()); }
    providers::Providers::None => {
      return Err(lperror::LovepackError::Error(format!("Couldn't find provider for {}",repo_url))); 
    }
  }
}