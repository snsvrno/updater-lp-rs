pub static REPO_URL : &str = r"https://github.com/([^/]*)/([^/]*)";
pub static API : &str = r"https://api.github.com";
pub static NAME : &str = "Github";

pub struct GithubParams {
  pub username: String,
  pub repo : String,
}