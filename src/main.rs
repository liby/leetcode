use std::{
    ffi::OsString,
    fs::{self, File},
    io::{BufRead, BufReader, Error, Write},
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

    let files_len = filename_vec.len();
    for i in 0..files_len {
        let format_path = format!("solutions/{}", filename_vec[i].to_str().unwrap());
        let file = BufReader::new(File::open(format_path)?);
        let title_line = file
            .lines()
            .map(|line| line.unwrap())
            .skip(3)
            .next()
            .unwrap();

        let format_filename = format!(
            "{}. [{}](https://github.com/liby/leetcode/blob/main/solutions/{})",
            conversion_filename_vec(&filename_vec[i])[0],
            conversion_title_vec(&title_line)[2..].join(" "),
            filename_vec[i].to_str().unwrap(),
        );
        write!(output, "{}\n", format_filename)?;
    }

    let commit_message = format!(
        "feat: ✨ add new solution of {}",
        conversion_filename_vec(&filename_vec[files_len - 1])[1]
    );

    Command::new("git").arg("add").arg('.').output().expect("shell exec error!");
    Command::new("git").arg("commit").arg('-m').arg(&commit_message).output().expect("shell exec error!");

    Ok(())
}
