extern crate updater_lp as updater;

#[test]
fn latest_version() {
    let ver = updater::get_latest_version("https://github.com/snsvrno/lpsettings-rs");

    println!("{:?}",ver);

    //assert!(false); // cannot know what the version is at the time of testing, have to manually check
    assert!(ver.is_ok());
}

#[test]
fn latest_version_link() {
    let ver = updater::get_link_for_latest("https://github.com/snsvrno/lpsettings-rs");

    println!("{:?}",ver);

    //assert!(false); // cannot know what the version is at the time of testing, have to manually check
    assert!(ver.is_ok());
}