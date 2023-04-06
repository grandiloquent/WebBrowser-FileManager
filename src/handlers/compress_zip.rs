use std::fmt::Error;
use std::path::{Path, PathBuf};
use rocket::serde::json::{json, Value};
use std::fs;
use rocket::futures::future::err;
use rocket::http::Status;
use rocket::serde::{json::Json};
use rocket::State;
use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
use zip::result::ZipError;
use zip::write::FileOptions;

use std::fs::File;
use walkdir::WalkDir;

fn zip_dir<T>(
    it: &mut dyn Iterator<Item=walkdir::DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
    where
        T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!("adding file {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}
const METHOD_STORED: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Stored);
// http://192.168.8.189:3000/compress_dir?path=D:\Books9
#[get("/compress_dir?<path>")]
pub async fn compress_zip(path: String) -> Result<(), Status> {
// https://doc.rust-lang.org/std/path/struct.Path.html
    match list_files(path.as_str()) {
        Ok(p) => {
            for file in p {
                if !file.is_dir() {
                    continue;
                }
                doit(file.to_str().unwrap(),
                     (file.to_str().unwrap().to_string()+".zip").as_str(),
                     METHOD_STORED.unwrap()
                );
            }
        }
        Err(_) => {
            return Err(Status::NotFound);
        }
    }
    Ok(())
}

fn list_files(path: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut list = Vec::new();
    let read_dir = fs::read_dir(path)?;
    for entry in read_dir {
        let dir_entry = entry?;
        list.push(dir_entry.path())
    }
    return Ok(list);
}
// https://github.com/zip-rs/zip/blob/master/examples/write_dir.rs
fn doit(
    src_dir: &str,
    dst_file: &str,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(path).unwrap();

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;

    Ok(())
}