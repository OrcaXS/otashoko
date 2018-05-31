use std::{self, io, fs};
use std::path::{Path, PathBuf};


extern crate zip;

pub fn read_zip(file: &Path, extract_path: &Path) -> Vec<String>
{
    let fname = std::path::Path::new(&*file);
    let zipfile = std::fs::File::open(&fname).unwrap();


    let mut archive = zip::ZipArchive::new(zipfile).unwrap();
    let mut contents = Vec::<String>::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = extract_path.join(file.sanitized_name());
        let path_str = outpath.as_path().to_str().unwrap();
        // if(&*file.name()).starts_with('_') {
        //     contents.push(format!("Ignored {}", path_str));
        // } else {
        //     contents.push(format!("{}", path_str));
        // }
        if (&*file.name()).ends_with('/') {
            // contents.push(format!("File {} extracted to \"{}\"", i, path_str));
            fs::create_dir_all(&outpath).unwrap();
        } else {
            // contents.push(format!("File {} extracted to \"{}\" ({} bytes)", i, path_str, file.size()));
            contents.push(format!("{}", path_str));
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    // let archive_file = zip::read::ZipFile{ data: archive, reader: zipfile };
    // let name = archive.name();

    
    // let mut file = match archive.by_name("test/lorem_ipsum.txt")
    // {
    //     Ok(file) => file,
    //     Err(..) => { println!("File test/lorem_ipsum.txt not found"); return 2;}
    // };

    // file.read_to_string(&mut contents).unwrap();
    // println!("{}", contents);

    return contents;
}
