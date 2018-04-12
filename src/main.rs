extern crate clap;
extern crate image;
extern crate rayon;
extern crate walkdir;

use std::fs::File;
use std::path::Path;
use std::result::Result;

use clap::{App, Arg};
use image::imageops::FilterType;
use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};

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

    let all_files: Vec<DirEntry> = WalkDir::new(source_dir)
        .into_iter()
        .filter_map(|f| f.ok())  // only files we have access to
        .filter(|f| !f.path().metadata().expect("failed to get dir metadata").is_dir())
        .collect();

    all_files
        .into_par_iter()
        .for_each(|source_path| {
            let destination_path = Path::new(destination_dir).join(source_path.path().file_name().unwrap());
            make_thumbnail(&source_path.path(), &destination_path, width, height)
                .unwrap_or_else(|e| println!("src: {:?}, dest: {:?}, error: {:?}", source_path, destination_path, e));
        });
}

fn make_thumbnail(src: &Path, dest: &Path, width: u32, height: u32) -> Result<(), image::ImageError> {
    let img = image::open(&src)?;
    let thumbnail = img.resize(width, height, FilterType::Lanczos3);
    let ref mut out = File::create(&dest)?;
    thumbnail.save(out, image::JPEG)
}
