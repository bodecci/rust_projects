use std::fs;
use std::io;

// to be able to cleanly exit from the real_main()
fn main() {
    std::process::exit(real_main())
}

// the actual logic takes place here
fn real_main() -> i32 {
    //create a vector called args to collect users input 
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usuage: {} <filename>", args[0]);
        return 1;
    }
     //args at the 2nd position, denoted by 1st index is the file name
    let fname = std::path::Path::new(&*args[1]);
    //open the file using standard fs
    let file = fs::File::open(&fname).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    //start from 0 and cover the entire length of archive
    // there will be multiple files in the zip archive and we need to extract all
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        //setting the path where the files will be extracted
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment:{}", i, comment);
            }
        }
        if (*file.name()).ends_with('/'){
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} exteracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            //if there is no parent for those files, create a new directory
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    0
}