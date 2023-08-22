use std::{
    env,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use clap::Parser;
use tracing::info;

const DOWNIE: &str = "Library/Containers/com.charliemonroe.Downie-4/Data/Library/Application \
                      Support/com.charliemonroe.Downie-4";

const KEY: &str = "CFBundleShortVersionString";

const PREMIUM_PLIST: &str = "/Applications/Navicat Premium.app/Contents/Info.plist";

#[derive(clap::Parser, Debug)]
struct Opt {
    /// reset downie
    #[arg(short, long, default_value_t = false)]
    downie: bool,

    /// reset premium
    #[arg(short, long, default_value_t = false)]
    premium: bool,

    /// reset sqlite
    #[arg(short, long, default_value_t = false)]
    sqlite: bool,
}

fn main() {
    env::set_var("RUST_LOG", "TRACE");
    tracing_subscriber::fmt::init();

    let opts = Opt::parse();
    info!("opts {:?}", opts);

    let home = env::var("HOME").unwrap();
    info!("current home dir is {:?}", &home);
    let home_dir = Path::new(home.as_str());
    reset_downie(home_dir, &opts);

    let val = read_plist(PREMIUM_PLIST).unwrap();
    info!("val is {:?}", &val);
}

fn reset_downie(home: &Path, opts: &Opt) {
    if !opts.downie {
        return;
    }
    let real_path = Path::join(home, DOWNIE);
    if let Err(err) = std::fs::remove_dir_all(real_path) {
        info!("clean error. {}", err);
    } else {
        info!("clean success");
    }
}

fn read_plist(pfile: &str) -> anyhow::Result<String> {
    let book = plist::Value::from_file(pfile)?;

    let title = book
        .as_dictionary()
        .and_then(|dict| dict.get(KEY))
        .and_then(|title| title.as_string());

    Ok(title.unwrap().to_string())
}
