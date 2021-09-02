use super::error::{Kind,Error};
use std::{path, fs::File, io::Write};
use std::process::Command;

pub fn handler(file: &Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
    println!("[in-toto] Got a new tar file to verify...");
    let dir = tempfile::tempdir()?;

    let file_path = &dir.path().join("in_toto.tar.gz");
    let mut f = File::create(file_path)?;
    f.write(file)?;
    drop(file);

    let filename = file_path.to_str().unwrap();
    untar(filename, &dir.path().to_str().unwrap())?;
    let res = check(&dir.path())?;
    dir.close()?;
    println!("[in-toto] Verification succeeded!");
    Ok(res)
}

fn untar(filename: &str, untar_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // We store the file contents to the local file 'in_toto.tar.gz'
    // and then untar.

    let res = Command::new("tar")
        .arg("-xf")
        .arg(filename)
        .arg("-C")
        .arg(untar_path)
        .status()?;

    match res.success() {
        true => Ok(()),
        false => Err(Box::new(Error::new(Kind::HandleFailed)))
    } 
}

fn check(path: &path::Path) -> Result<String, Box<dyn std::error::Error>> {
    // TODO : assign specific .pub and .layout
    // Here we think by default: 
    // layout : root.layout
    // pubkey : jerry.pub
    // 
    // TODO : update according to https://github.com/in-toto/in-toto-rs
    //
    // Warning: now in-toto just gives a framework to verify
    // different steps, but without providing a concrete way to get sha256 hash
    // of the artifact. 
    // We here directly give out the sha256 hash when passing the test.
    // So, the program ONLY can be used in a demo!

    let res = Command::new("in-toto-verify")
    .current_dir(path)
        .arg("--layout")
        .arg("root.layout")
        .arg("--layout-key")
        .arg("jerry.pub")
        .status()?;

    // 
    match res.success() {
        true => Ok("48772e82a2993f44894820637ce13e0aceb9ab68d3b01dab79c945eaaa2d74cf".to_string()),
        false => Err(Box::new(Error::new(Kind::HandleFailed)))
    } 
}