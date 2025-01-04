use chrono::{DateTime, FixedOffset};
use exempi2::{PropFlags, SerialFlags, XmpFile};
use std::path::PathBuf;

const EXIF_SCHEMA: &str = "http://ns.adobe.com/exif/1.0/";
const DUBLIN_CORE_SCHEMA: &str = "http://purl.org/dc/elements/1.1/";
const XMP_SCHEMA: &str = "http://ns.adobe.com/xap/1.0/";
const XMP_TIME_FORMATS: [&str; 9] = [
    "%FT%T%.f%:z",
    "%FT%T%.f",
    "%FT%T%:z",
    "%FT%T",
    "%FT%H:%M%:z",
    "%FT%H:%M",
    "%F",
    "%Y-%m",
    "%Y",
];
const EXIF_TIME_FORMAT: &str = "%FT%T%.3f%:z";

const TIME_FORMATS: &'static [&'static str] = &[EXIF_TIME_FORMAT]; // pointer to the array because
                                                                   // consts get copied on use

struct File {
    path: PathBuf,
    date: Option<chrono::DateTime<FixedOffset>>,
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
        println!(
            "{}",
            xmp.serialize(SerialFlags::empty(), 2)
                .unwrap()
                .to_str()
                .unwrap()
        );

        let date: Option<DateTime<chrono::FixedOffset>> = {
            let exif_date = xmp.get_property(
                EXIF_SCHEMA,
                "exif:DateTimeOriginal",
                &mut PropFlags::empty(),
            );
            let xmp_date = xmp.get_property(XMP_SCHEMA, "xmp:CreateDate", &mut PropFlags::empty());
            let dublin_core_date =
                xmp.get_property(DUBLIN_CORE_SCHEMA, "dc:created", &mut PropFlags::empty());
            let mut dates: [Option<DateTime<FixedOffset>>; 3] = [None; 3];
            match exif_date {
                Ok(property) => {
                    if let Ok(parsed) =
                        DateTime::parse_from_str(property.to_str().unwrap(), EXIF_TIME_FORMAT)
                    {
                        dates[0] = Some(parsed);
                    };
                }
                Err(_) => {
                    todo!()
                }
            }
            match xmp_date {
                Ok(property) => {
                    if let Some(parsed) = parse_xmp_date(property.to_str().unwrap()) {
                        //what
                        dates[1] = Some(parsed);
                    }
                }
                Err(_) => {
                    todo!()
                }
            }
            match dublin_core_date {
                Ok(property) => todo!(),
                Err(_) => todo!(),
            }
            todo!()
        };

        let mut file: File = File {
            path,
            date,
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

fn parse_xmp_date(date: &str) -> Option<DateTime<FixedOffset>> {
    let check = |format: &str| {
        if let Ok(parsed) = DateTime::parse_from_str(date, format) {
            Some(parsed)
        } else {
            None
        }
    };

    for format in XMP_TIME_FORMATS {
        if let Some(parsed) = check(format) {
            return Some(parsed);
        }
    }
    None
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
    //benchmark!({ File::read(PathBuf::from("./testing/test.jpg")) }, 10000);
    //benchmark!(println!("test"), 1000);
}
