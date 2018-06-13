use regex;
use structs::providers::Providers;

static REPO_URL : &str = r"https://([^/]*)/([^/]*)/([^/]*)";

pub struct RepoBreakdown {
  pub provider : Providers,
  pub username: String,
  pub repo : String,
  pub url : String,
}

impl RepoBreakdown {
  pub fn new(repo_url : &str) -> RepoBreakdown {
    let re = regex::Regex::new(REPO_URL).unwrap();

    let mut _provider = Providers::None;
    let mut _username : Option<String> = None;
    let mut _repository : Option<String> = None;

    if let Some(captures) = re.captures(repo_url){
      return RepoBreakdown {
        url : repo_url.to_string(),
        username : if let Some(user) = captures.get(2) { user.as_str().to_string() } else { "".to_string() },
        repo : if let Some(repo) = captures.get(3) { repo.as_str().to_string() } else { "".to_string() },
        provider : if let Some(provider_string) = captures.get(1) { match provider_string.as_str() {
            "github.com" => Providers::Github,
            _ => Providers::None,
          }
        } else { Providers::None },
        
      };
    } else { return RepoBreakdown::empty(repo_url); }
  }

  pub fn empty(repo_url : &str) -> RepoBreakdown {
    RepoBreakdown {
      url : repo_url.to_string(),
      username: "".to_string(),
      repo: "".to_string(),
      provider: Providers::None
    }
  }

  pub fn is_valid(&self) -> bool {
    true
  }
}