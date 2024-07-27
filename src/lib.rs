use std::fs::{self, OpenOptions, File};
use std::io::{Write, BufRead, BufReader};
use std::error::Error;
use std::process;
use colored::*;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let name = read_line_from_todo_noprint()?;
    Config::order(&config, name)?;
    title()?;
    read_line_from_todo()?;
    
    Ok(()) 
}
pub struct Config {
    pub command: String,
    pub todo: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 1 {
            return Err("not enough arguments");
        }

        let command = args[0].clone();
        let todo: String = args[1..].join(" ");

        Ok(Config { command, todo })
    }


    pub fn order(config: &Config, name: Vec<String>) -> Result<(), Box<dyn Error>> {
        let file = "todo.txt";
        let todo = config.todo.clone();
        
        if config.command == "add" {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(file)?;
            writeln!(file, "{}", todo)?;
        } else if config.command == "delete" || config.command == "del" {
            let contents = fs::read_to_string(file)?;
            let line_number: usize = search_case_insensitive(&config.todo, &contents);
            delete_line("todo.txt", line_number)?;
        } else if config.command == "help" {
            cleanup_previous_downloads()?;
            let _help = help()?;
        } else if config.command == "play" {
            play(name)?;
        } else {
            process::exit(1);
        }

        Ok(())
    }



}

fn play(name: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    for (a, i) in name.iter().enumerate() {
        print!("\x1B[2J\x1B[1;1H");
        read_playing(a as i32)?;
        cleanup_previous_downloads()?;
        let string = i.to_string();
        let video_url = search_and_get_url(&string).expect("a");

        let file_path = download_audio(&video_url)?;
    
        let mp3_path = convert_to_mp3(&file_path)?;
    
        let _xx = play_audio(&mp3_path);
    
        fs::remove_file(&file_path)?;
        fs::remove_file(&mp3_path)?;

    }
    print!("\x1B[2J\x1B[1;1H");

    Ok(())
    

}

fn cleanup_previous_downloads() -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "webm" || ext == "mp3" || ext == "opus" || ext == "m4a" {
                fs::remove_file(path)?;
            }
        }
    }
    Ok(())
}

fn search_and_get_url(query: &str) -> Result<String, Box<dyn std::error::Error>> {
    let search_term = format!("ytsearch1:{}", query);
    
    let output = Command::new("yt-dlp")
        .args(&["--get-id", "--no-playlist", &search_term])
        .output()?;


    let video_id = String::from_utf8(output.stdout)?.trim().to_string();
    Ok(format!("https://www.youtube.com/watch?v={}", video_id))
}



fn download_audio(video_url: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let output_template = format!("%(title)s_{}.%(ext)s", timestamp);
    
    let _output = Command::new("yt-dlp")
        .args(&[
            "-x",
            "--audio-format", "best",
            "-o", &output_template,
            video_url,
        ])
        .output()?;

    let downloaded_file = fs::read_dir(".")?
        .filter_map(Result::ok)
        .find(|entry| {
            let name = entry.file_name();
            let name_str = name.to_str().unwrap_or("");
            name_str.contains(&format!("_{}", timestamp))
        })
        .ok_or("cannot find music file")?;

    Ok(downloaded_file.path())
}

fn convert_to_mp3(input_path: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let output_path = input_path.with_extension("mp3");
    
    let output = Command::new("ffmpeg")
        .args(&["-i", input_path.to_str().unwrap(), "-acodec", "libmp3lame", "-b:a", "256k", output_path.to_str().unwrap()])
        .output()?;

    if output.status.success() {
        Ok(output_path)
    } else {
        Err("ffmpeg convert failed".into())
    }
}

fn play_audio(file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("afplay")
        .arg(file_path)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err("failed".into())
    }
}
pub fn title() -> Result<(), Box<dyn Error>> {
    let title = "

    🌈Playlist✨
    
    ";
    println!("{}", title.bold().bright_magenta());
    
    Ok(())
}

pub fn read_line_from_todo() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("todo.txt")?;
    for (index, line) in contents.lines().into_iter().enumerate() {
        let index: i32 = index.to_string().trim().parse().expect("s");
        let index = (index+1).to_string();
        println!("  🩵{}{}{} - {}", "[".bright_cyan().bold(), index.bright_yellow(), "]".bright_cyan().bold(), line.italic());
        
    }
    println!();
    println!();

    Ok(())
}

pub fn read_playing(number: i32) -> Result<(), Box<dyn Error>> {
    title()?;
    let contents = fs::read_to_string("todo.txt")?;
    for (index, line) in contents.lines().into_iter().enumerate() {
        let index: i32 = index.to_string().trim().parse().expect("s");
        if number == index as i32 { 
            let index = (index+1).to_string();
            println!("  🩷{}{}{} {} {}", "[".bright_purple().bold(), index.bright_green(), "]".bright_purple().bold(), "-".bright_green(),line.italic().bright_green());
        }
        else {
            let index = (index+1).to_string();
            println!("  🩵{}{}{} - {}", "[".bright_cyan().bold(), index.bright_yellow(), "]".bright_cyan().bold(), line.italic());
        }
        
       
        
    }
    println!();
    println!();
    println!("  {}", "In queued - now on playing 💿".bright_purple().bold());
    


    Ok(())
}

pub fn read_line_from_todo_noprint() -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string("todo.txt")?;
    let mut output = vec![];
    for line in contents.lines().into_iter() {
        output.push(line.into());
    }

    Ok(output)
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn search_case_insensitive<'a>(
    query: &String,
    contents: &'a str,
) -> usize {
    let query = query.to_lowercase();
    let mut results: usize = 255;

    for (index, line) in contents.lines().into_iter().enumerate() {
        if line.to_lowercase().contains(&query) {
            results = index;
            break;
        }
    }

    results
}


pub fn delete_line(file_path: &str, line_number: usize) -> std::io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            if index != line_number {
                line.ok()
            } else {
                None
            }
        })
        .collect();

    let mut file = File::create(file_path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

pub fn help() -> Result<(), Box<dyn Error>> {
    clear_screen();

    println!("
command line

    add              - adding the song into playlist
    delete || del    - deleteing the song
    play             - playing playlist (you cannot add more song while playing.)

    *anything        - exit from program
    ctrl + c         - to quit while on playing, you have to exit from program by ctrl + c

");
    


    Ok(())
}