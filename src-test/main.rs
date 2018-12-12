extern crate updater_lp as updater;
extern crate version_lp as version; use version::Version;

use std::env;
use std::fs;

static PATH : &str = "https://github.com/snsvrno/lpsettings-rs";

fn main() {
    // choose an older version to test updating
    let this_version = Version::new(&[0,1,4]);

    println!("running version {}",this_version);

    println!("checking for update ...");
    match updater::get_latest_version(PATH) {
        Err(error) => println!("{}",error),
        Ok(latest) => {
            println!("latest version is {}",latest);
            if latest > this_version {
                println!("update available, overriding with 0.1.5");
                if let Err(error) = updater::update_with_version(PATH,&Version::new(&[0,1,5])){
                    println!("ERROR : {}",error);
                }
                
            } else {
                println!("no update available.");
            }
        }
    }
}