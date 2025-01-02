use chrono::FixedOffset;
use exempi2::{PropFlags, XmpFile};
use std::path::PathBuf;

const EXIF_SCHEMA: &str = "http://ns.adobe.com/exif/1.0/";
const DUBLIN_CORE_SCHEMA: &str = "http://purl.org/dc/elements/1.1/";
const EXIF_TIME_FORMAT: &str = "%FT%T%.3f%:z";

struct File {
    path: PathBuf,
    date: chrono::DateTime<FixedOffset>,
    tags: Vec<String>,
}
impl File {
    fn read(path: PathBuf) -> Result<File, String> {
        let file = match XmpFile::new_from_file(&path, exempi2::OpenFlags::READ) {
            Ok(file) => file,
            Err(err) => return Err(format!("{}", err)),
        };

        let xmp = match file.get_new_xmp() {
            Ok(xmp) => xmp,
            Err(err) => return Err(format!("{}", err)),
        };

        let mut file: File = File {
            path,
            date: chrono::DateTime::parse_from_str(
                xmp.get_property(
                    EXIF_SCHEMA,
                    "exif:DateTimeOriginal",
                    &mut PropFlags::empty(),
                )
                .unwrap()
                .to_str()
                .unwrap(),
                EXIF_TIME_FORMAT,
            )
            .unwrap(),
            tags: Vec::new(),
        };

        let mut i: i32 = 1;
        loop {
            match xmp.get_array_item(DUBLIN_CORE_SCHEMA, "dc:subject", i, &mut PropFlags::empty()) {
                Ok(tag) => file.tags.push(tag.to_str().unwrap().into()),
                Err(_) => break,
            }
            i += 1;
        }

        Ok(file)
    }
}

macro_rules! benchmark {
    ($func:expr, $num:expr) => {
        let now = std::time::Instant::now();
        for _ in 0..$num {
            let _ = $func;
        }
        println!("{}", now.elapsed().as_millis());
    };
}

fn main() {
    println!("Hello, world!");
    match File::read(PathBuf::from("./testing/test.jpg")) {
        Ok(file) => {
            dbg!(file.tags);
        }
        Err(_) => {}
    }
    benchmark!({ File::read(PathBuf::from("./testing/test.jpg")) }, 10000);
    //benchmark!(println!("test"), 1000);
}
