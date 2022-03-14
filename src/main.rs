use std::{
    ffi::OsString,
    fs::{self, File},
    io::{BufRead, BufReader, Error, Write},
    process::Command,
};

fn conversion_title_vec(lines_iter: &str) -> Vec<&str> {
    lines_iter.split_ascii_whitespace().collect::<Vec<&str>>()
}

fn conversion_filename_vec(filename: &OsString) -> Vec<&str> {
    filename.to_str().unwrap().split('.').collect()
}

fn main() -> Result<(), Error> {
    println!("Rust Solution!");

    let mut filename_vec: Vec<OsString> = Vec::new();
    match fs::read_dir("solutions") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                filename_vec.push(path.unwrap().file_name());
            }
            filename_vec.sort_by(|a, b| {
                conversion_filename_vec(a)[0]
                    .parse::<i32>()
                    .unwrap()
                    .cmp(&conversion_filename_vec(b)[0].parse::<i32>().unwrap())
            });
        }
    };

    let path = "README.md";
    let mut output = File::create(path)?;
    write!(output, "# Rust Solution\n\n")?;

    let mut commit_message_vec = vec![String::from("feat: ✨ add new solutions")];
    let files_len = filename_vec.len();
    for filename in filename_vec.iter().take(files_len) {
        let format_path = format!("solutions/{}", filename.to_str().unwrap());
        let file = BufReader::new(File::open(format_path.clone())?);
        let title_line = file.lines().map(|line| line.unwrap()).nth(3).unwrap();

        let format_filename = format!(
            "{}. [{}](https://github.com/liby/leetcode/blob/main/solutions/{})",
            conversion_filename_vec(filename)[0],
            conversion_title_vec(&title_line)[2..].join(" "),
            filename.to_str().unwrap(),
        );
        writeln!(output, "{}", format_filename)?;

        let metadata = fs::metadata(format_path).unwrap();
        let creation_time = metadata.created()?;
        if creation_time.elapsed().unwrap().as_secs() < (2 * 3600) && metadata.is_file() {
            commit_message_vec.push(format!(
                "feat: ✨ add new solution of {}",
                conversion_filename_vec(filename)[1]
            ));
        }
    }

    let commit_message = if commit_message_vec.len() > 2 {
        &commit_message_vec[0]
    } else {
        &commit_message_vec[1]
    };
    Command::new("git")
        .arg("add")
        .arg("--all")
        .output()
        .expect("shell exec error!");
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output()
        .expect("shell exec error!");

    Ok(())
}
