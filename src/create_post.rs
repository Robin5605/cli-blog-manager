use std::{io::{self, Write}, fs::File, path::{Path}};

use crate::utils::get_fmt_datetime;

pub fn create_post(filename: &String) -> Result<(), io::Error> {
    let mut vec_buf: Vec<String> = Vec::new();

    vec_buf.push(String::from("---"));
    vec_buf.push(String::from("title: "));
    vec_buf.push(String::from("description: "));
    vec_buf.push(String::from("tags: "));
    vec_buf.push(format!("created_at: {}", get_fmt_datetime()));
    vec_buf.push(String::from("---"));
    vec_buf.push(String::new());

    let path = Path::new(".").join(format!("{}.md", filename));
    let mut file = File::create(path)?;
    file.write_all(vec_buf.join("\n").as_bytes())?;

    Ok(())
}