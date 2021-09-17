use std::{
    ffi::OsString,
    fs::{self, File},
    io::{BufRead, BufReader, Error, Write},
};

fn conversion_title_vec(lines_iter: &str) -> Vec<&str> {
    lines_iter.split_ascii_whitespace().collect::<Vec<&str>>()
}
fn main() -> Result<(), Error> {
    println!("Rust Solution!");

    let mut v: Vec<OsString> = Vec::new();
    match fs::read_dir("solutions") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                v.push(path.unwrap().file_name());
            }
            v.sort_by(|a, b| {
                a.to_str().unwrap().split('.').collect::<Vec<&str>>()[0]
                    .parse::<i32>()
                    .unwrap()
                    .cmp(
                        &b.to_str().unwrap().split('.').collect::<Vec<&str>>()[0]
                            .parse::<i32>()
                            .unwrap(),
                    )
            });
        }
    };

    let path = "README.md";
    let mut output = File::create(path)?;
    write!(output, "# Rust Solution\n\n")?;

    for i in 0..v.len() {
        let split_filename = v[i].to_str().unwrap().split('.').collect::<Vec<&str>>();
        let format_path = format!("solutions/{}", v[i].to_str().unwrap());
        let file = BufReader::new(File::open(format_path)?);
        let title_line = file
            .lines()
            .map(|line| line.unwrap())
            .skip(3)
            .next()
            .unwrap();

        let format_filename = format!(
            "{}. [{}](https://github.com/liby/leetcode/blob/main/solutions/{})",
            split_filename[0],
            conversion_title_vec(&title_line)[2..].join(" "),
            v[i].to_str().unwrap(),
        );
        write!(output, "{}\n", format_filename)?;
    }

    Ok(())
}
