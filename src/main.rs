use error_chain::error_chain;
use flate2::read::GzDecoder;
use serde_json;
use std::fs::File;
use std::path::Path;
use tar::Archive;

error_chain! {
  foreign_links {
    Io(std::io::Error);
    StripPrefixError(::std::path::StripPrefixError);
  }
}

fn main() -> Result<()> {
    let file = File::open("takeout.tgz")?;
    let mut archive = Archive::new(GzDecoder::new(file));
    let records_file_path = Path::new("Takeout/Location History/Records.json");

    println!("Extracted the following files:");
    archive
        .entries()?
        .filter_map(|e| e.ok())
        .filter(|entry| records_file_path == entry.path().unwrap())
        .for_each(|entry| {
            let path2 = entry.path().unwrap().to_owned();
            println!("Path: {}", path2.display());

            let deserializer = serde_json::Deserializer::from_reader(entry);
            let iterator = deserializer.into_iter::<serde_json::Value>();
            for item in iterator {
                println!("Got {:?}", item.unwrap());
            }
        });

    Ok(())
}
