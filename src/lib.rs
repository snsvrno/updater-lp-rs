extern crate lperror;
extern crate version; use version::version::Version;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
extern crate regex;
extern crate ansi_term;
extern crate restson;
extern crate chrono;

extern crate lpsettings;

mod structs; use structs::repo_breakdown::RepoBreakdown;
mod providers;

use std::path::PathBuf;

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

pub fn check_for_updates(app_name : &str, repo_url : &str) -> Result<Option<String>,lperror::LovepackError> {
  let mut should_check = false;
  let mut new_version : Option<Version> = None;

  match lpsettings::get_value(&format!("updates.{}.last",&app_name)) {
    None => should_check = true,
    Some(raw_date) => { 
      if let Ok(date) = chrono::DateTime::parse_from_str(&raw_date,"%Y-%m-%d %H%M%S%z") {
        if date.date() == chrono::prelude::Local::today() {
          match lpsettings::get_value(&format!("updates.{}.available",&app_name)) {
            None => { },
            Some(raw_version) => new_version = Version::from_str(&raw_version)
          }
        } else {
          should_check = true;
        }
      }
    },
  }

  if should_check {
    if let Some(version) = Version::from_str(env!("CARGO_PKG_VERSION")) {
      match has_updates(&version,&repo_url) {
        Err(error) => { return Err(error); }
        Ok(found_version) => {
          lpsettings::set_value_global(&format!("updates.{}.last",&app_name), &chrono::prelude::Local::now().format("%Y-%m-%d %H%M%S%z").to_string() );
          match found_version {
            None => return Ok(None),
            Some(version) => { 
              lpsettings::set_value_global(&format!("updates.{}.available",&app_name), &version.to_string() );
              return Ok(Some(version.to_string()))
            }
          }
        }
      }
    } else {
      return Err(lperror::LovepackError::Error(format!("Error parsing {}'s version: {}",&app_name,env!("CARGO_PKG_VERSION"))));
    }  
  }

  return Ok(if let Some(version) = new_version {Some(version.to_string())} else {None});
}