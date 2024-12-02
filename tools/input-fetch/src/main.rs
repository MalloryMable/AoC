use time::{OffsetDateTime, UtcOffset, Month};
use std::{env, fs};
use std::path::Path;
use std::process;
use ureq::get;

const BASE_URL: &str = "https://adventofcode.com";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = env::var("AOC_SESSION").expect("Please set AOC_SESSION enviroment variable");

    let args: Vec<String> = env::args().collect();
    let (year, day) = get_target_date(&args);

    if day < 1 || day > 25 {
        eprintln!("Day must be 1-25");
        process::exit(1);
    }


    // Create a year and day directory if one doesn't exist. Appends leading zeros to day when needed
    let day_dir = format!("{}/day-{:02}", year, day);
    fs::create_dir_all(&day_dir)?;

    let url = format!("{}/{}/day/{}/input", BASE_URL, year, day);
    let response = get(&url)
        .set("Cookie", &format!("session={}", session))
        .call()?;

    if response.status() != 200 {
        eprintln!("Failed to fetch input: {}", response.status());
        process::exit(1);
    }
    

    let input = response.into_string()?;
    let file_path = Path::new(&day_dir).join("input.txt");
    fs::write(file_path, input)?;

    println!("Successfully downloaded input for day {} of {} to {}", day, year, day_dir);

    Ok(())
}

fn get_target_date(args: &[String]) -> (i32, u8) {
    
    let est_offset: UtcOffset = UtcOffset::from_hms(-5, 0, 0).unwrap();
    let now = OffsetDateTime::now_utc().to_offset(est_offset);
    
    // TODO: If the month of the year is before december default to last year
    let year = if args.len() > 2 {
        args[2].parse().unwrap_or(now.year())
    } else {
        now.year()
    };

    let day = if args.len() > 1 {
        args[1].parse().unwrap_or_else(|_| {
            if now.month() == Month::December && now.day() <= 25 {
                now.day()
            } else {
                eprintln!("Please specify a day (1-25)");
                process::exit(1);
            }
        })
    } else if now.month() == Month::December && now.day() <= 25 {
        now.day()
    } else {
        eprintln!("Please Specify a day (1-25)");
        process::exit(1);
    };

    (year, day)
}

