use ureq;
use std::fs::{OpenOptions, remove_file};
use std::io::Write;
use std::process::{Command};
use std::env;
use std::io::Read;

const WEEKLY:&str = "weekly";
const DAILY:&str = "daily";
const RANDOM:&str = "";

fn main() {
    let args: Vec<String> = env::args().collect();

    let search_type =
        get_arg(&args, 1, search_type_from_str, ImageSearchType::Random);

    let dimensions = get_arg(&args, 2,
                             |x| -> String{ x.to_owned() }, "1920x1080".to_string());

    let query = if args.len() >= 4 {
        let mut search_query = String::new();
        const START:usize = 3;
        for i in START..args.len() {
            if i != START {
                search_query.push_str(" ");
            }

            search_query.push_str(args.get(i).unwrap());
        }

        search_query
    } else {
        String::from("nature")
    };

    println!("{}", query.as_str());

    let url = format!("https://source.unsplash.com/{}/{}?{}", dimensions, str_from_search_type(search_type), query);
    let resp = ureq::get(url.as_str()).call().expect("unable to perform GET request");

    let len = resp.header("Content-Length")
        .and_then(|s| s.parse::<usize>().ok())
        .expect("unable to calculate body content-length");

    let mut bytes: Vec<u8> = Vec::with_capacity(len);
    resp.into_reader()
        .read_to_end(&mut bytes)
        .expect("unable to read response body");

    let temp_file_path = std::env::temp_dir().join("dailywallpaper.jpg");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&temp_file_path)
        .expect("unable to create temporary wallpaper file");

    file.write_all(&*bytes).expect("unable to write image to file");

    let status = Command::new("feh")
        .arg("--bg-fill")
        .arg(&temp_file_path)
        .status()
        .expect("Failed to apply wallpaper");

    if !status.success() {
        println!("Failed to apply wallpaper")
    }

    remove_file(&temp_file_path).expect("unable to remove temporary wallpaper file");
}

fn get_arg<T>(args: &Vec<String>, arg_index: usize,
              format_func: fn(&String) -> T, default: T) -> T {

    let result:T = if args.len() >= (arg_index + 1) {
        format_func(args.get(arg_index).unwrap())
    } else {
        default
    };

    result
}

fn search_type_from_str(input: &String) -> ImageSearchType {
    match input.to_lowercase().as_str() {
        RANDOM => ImageSearchType::Random,
        DAILY => ImageSearchType::Daily,
        WEEKLY => ImageSearchType::Weekly,
        _ => ImageSearchType::Random
    }
}

fn str_from_search_type(search: ImageSearchType) -> String {
    match search {
        ImageSearchType::Random => RANDOM,
        ImageSearchType::Daily => DAILY,
        ImageSearchType::Weekly => WEEKLY
    }.to_string()
}

enum ImageSearchType {
    Random,
    Daily,
    Weekly,
}