use sources::github::{ asset::Asset, params::GithubParams };

#[derive(Serialize,Deserialize,Debug)]
pub struct GitRest { 
    pub url: String,
    pub tag_name: String,
    pub assets: Vec<Asset>
}

// for getting the latest one
impl<'a> restson::RestPath<&'a GithubParams> for GitRest {
    fn get_path(repo: &'a GithubParams) -> Result<String,restson::Error> { 
        Ok(format!("repos/{}/{}/releases/latest",repo.username,repo.repo)) 
    }
}

// for getting all of them
impl<'a> restson::RestPath<&'a GithubParams> for Vec<GitRest> {
    fn get_path(repo: &'a GithubParams) -> Result<String,restson::Error> { 
        Ok(format!("repos/{}/{}/releases",repo.username,repo.repo)) 
    }
}
