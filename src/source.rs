use failure::Error;

use traits::provider::Provider;

use sources::github::Github;

pub fn get_provider(link : &str) -> Result<Box<dyn Provider>,Error> {
    
    let github = Github { repo_link : link.to_string() };
    if github.valid_url() {
        return Ok(Box::new(github));
    }

    Err(format_err!("No valid provider found for : {}",link))
}