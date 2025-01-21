use chrono::{DateTime, FixedOffset};
use exempi2::{PropFlags, SerialFlags, XmpFile, XmpString};
use std::path::PathBuf;

const EXIF_SCHEMA: &str = "http://ns.adobe.com/exif/1.0/";
const DUBLIN_CORE_SCHEMA: &str = "http://purl.org/dc/elements/1.1/";
const XMP_SCHEMA: &str = "http://ns.adobe.com/xap/1.0/";
//const XMP_TIME_FORMATS: [&str; 9] = [
//    "%FT%T%.f%:z",
//    "%FT%T%.f",
//    "%FT%T%:z",
//    "%FT%T",
//    "%FT%H:%M%:z",
//    "%FT%H:%M",
//    "%F",
//    "%Y-%m",
//    "%Y",
//];
//const EXIF_TIME_FORMAT: &str = "%FT%T%.3f%:z";
//
//const TIME_FORMATS: &'static [&'static str] = &[EXIF_TIME_FORMAT]; // pointer to the array because
                                                                   // consts get copied on use

#[derive(Debug)]
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
        //println!(
        //    "{}",
        //    xmp.serialize(SerialFlags::empty(), 2)
        //        .unwrap()
        //        .to_str()
        //        .unwrap()
        //);

        let date: Option<DateTime<chrono::FixedOffset>> = {
            let exif_date = xmp.get_property(
                EXIF_SCHEMA,
                "exif:DateTimeOriginal",
                &mut PropFlags::empty(),
            ).unwrap_or_default();
            let xmp_date = xmp.get_property(XMP_SCHEMA, "xmp:CreateDate", &mut PropFlags::empty()).unwrap_or_default();
            let dublin_core_date =
                xmp.get_property(DUBLIN_CORE_SCHEMA, "dc:created", &mut PropFlags::empty()).unwrap_or_default();

            let dates: [Result<DateTime<FixedOffset>, (chrono::ParseError, chrono::ParseError)>; 3] = [
                parse_date(xmp_date.to_str().unwrap()),
                parse_date(exif_date.to_str().unwrap()),
                parse_date(dublin_core_date.to_str().unwrap())
            ];
            
            let mut ret: Option<DateTime<chrono::FixedOffset>> = None;
            for value in dates {
                if let Ok(date) = value {
                    // log that there are multiple dates maybe
                    ret = Some(date);
                }
            }
            ret
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

fn parse_date(date: &str) -> Result<DateTime<FixedOffset>, (chrono::ParseError, chrono::ParseError)> {
    let rfc3339 = DateTime::parse_from_rfc3339(date);
    let rfc2822: Result<DateTime<FixedOffset>, chrono::ParseError>;
    if let Ok(parsed) = rfc3339 {
        return Ok(parsed)
    } else {
        rfc2822 = DateTime::parse_from_rfc2822(date);
        if let Ok(parsed) = rfc2822 {
            return Ok(parsed);
        }
    }
    Err((rfc3339.err().unwrap(), rfc2822.err().unwrap()))
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
            dbg!(file);
        }
        Err(_) => {}
    }
    //benchmark!({ File::read(PathBuf::from("./testing/test.jpg")) }, 10000);
    //benchmark!(println!("test"), 1000);
}
