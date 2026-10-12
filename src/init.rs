use std::{fs, io, path::Path};

pub fn init() -> Result<(), io::Error> {
    let path = Path::new(".bs");

    fs::create_dir(path)?;
    fs::create_dir(path.join("objects"))?;
    fs::create_dir(path.join("refs"))?;
    fs::write(path.join("HEAD"), "ref: refs/heads/master\n")?;
    println!("Initialized blobstore directory");
    Ok(())
}
