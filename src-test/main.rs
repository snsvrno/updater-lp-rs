//! an example program to show how updating works. If you run this
//! it should replace itself with the 0.1.5 release of lpsettings.
//! 
//! will only work on windows and linux because there is no mac release
//! for lpsettings.
//! 
//! technically the program will still 'work' but it will error on not
//! being able to find a version.

extern crate updater_lp as updater;

use std::env;
use std::fs;

static PATH : &str = "https://github.com/snsvrno/lpsettings-rs";

fn main() {
    // choose an older version to test updating
    let this_version = updater::create_version(&[0,1,4]);
    println!("running version {}",this_version);

    println!("checking for update ...");
    match updater::get_latest_version(PATH) {
        Err(error) => println!("ERROR : {}",error),
        Ok(latest) => {

            println!("latest version is {}",latest);

            if latest > this_version {
                println!("update available, overriding with 0.1.5");

                // overriding with a different version because the latest version doesn't have 
                // a release for the platforms I'm developing on. You should be using 
                // `latest` instead.
                let override_version = updater::create_version(&[0,1,5]);
                if let Err(error) = updater::update_with_version(PATH,&override_version){
                    println!("ERROR : {}",error);
                }
                
            } else {
                println!("no update available.");
            }
        }
    }
}