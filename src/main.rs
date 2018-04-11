extern crate clap;
extern crate image;
extern crate walkdir;

use std::fs::{canonicalize, File};
use std::path::Path;
use std::result::Result;
use std::thread;

use clap::{App, Arg};
use image::imageops::FilterType;
use walkdir::WalkDir;

fn main() {
    let args = App::new("Thumberst")
        .version("0.1.0")
        .about("Creates thumbnails from images (also in subdirs)")
        .arg(Arg::with_name("src_dir")
                .long("src_dir")
                .takes_value(true)
                .required(true)
                .help("Source dir"))
        .arg(Arg::with_name("dest_dir")
                .long("dest_dir")
                .takes_value(true)
                .required(true)
                .help("Destination dir"))
        .arg(Arg::with_name("thumbnail_width")
                .long("width")
                .takes_value(true)
                .help("Thumbnail width"))
        .arg(Arg::with_name("thumbnail_height")
                .long("height")
                .takes_value(true)
                .help("Thumbnail height"))
        .get_matches();

    let source_dir = args.value_of("src_dir").expect("Unable to parse src_dir");
    let destination_dir = args.value_of("dest_dir").expect("Unable to parse dest_dir");
    let width = args.value_of("thumbnail_width").unwrap_or("120").parse::<u32>().unwrap();
    let height = args.value_of("thumbnail_height").unwrap_or("120").parse::<u32>().unwrap();

    let all_files = WalkDir::new(source_dir)
        .into_iter()
        .filter_map(|f| f.ok())  // only files we have access to
        .filter(|f| !f.path().metadata().expect("failed to get dir metadata").is_dir());

    let abspath = |path: &Path| { canonicalize(path).expect("Failed to get abspath") };

    for rel_path in all_files {
        let source_path = abspath(&rel_path.path());
        let destination_path = abspath(&Path::new(destination_dir).join(rel_path.path().file_name().unwrap()));
        let handle = thread::spawn(move || {
            make_thumbnail(&source_path, &destination_path, width, height)
                .unwrap_or_else(|e| println!("src: {:?}, dest: {:?}, error: {:?}", source_path, destination_path, e));
        });
        handle.join().unwrap();
    }
}

fn make_thumbnail(src: &Path, dest: &Path, width: u32, height: u32) -> Result<(), image::ImageError> {
    let img = image::open(src.to_str().unwrap())?;
    let thumbnail = img.resize(width, height, FilterType::Lanczos3);
    let ref mut out = File::create(dest.to_str().unwrap())?;
    thumbnail.save(out, image::JPEG)
}
