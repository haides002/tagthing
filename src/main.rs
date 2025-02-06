use chrono::{DateTime, Datelike, FixedOffset, Timelike};
use exempi2::{PropFlags, SerialFlags, Xmp, XmpFile, XmpString};
use std::path::PathBuf;

const EXIF_SCHEMA: &str = "http://ns.adobe.com/exif/1.0/";
const DUBLIN_CORE_SCHEMA: &str = "http://purl.org/dc/elements/1.1/";
const XMP_SCHEMA: &str = "http://ns.adobe.com/xap/1.0/";

#[derive(Debug)]
struct File {
    pub path: PathBuf,
    pub date: Option<chrono::DateTime<FixedOffset>>,
    pub tags: Vec<String>,
}
impl File {
    pub fn read(path: PathBuf) -> Result<File, String> {
        let file = match XmpFile::new_from_file(&path, exempi2::OpenFlags::ONLY_XMP) {
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
                    // maybe do some more intelligent date selection?
                    //println!("Date found: {}", date.to_string());
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
        //println!("{}", xmp.serialize(SerialFlags::empty(), 2).unwrap());
        loop {
            match xmp.get_array_item(DUBLIN_CORE_SCHEMA, "dc:subject", i, &mut PropFlags::empty()) {
                Ok(tag) => 
                    file.tags.push(tag.to_str().unwrap().into())
                ,
                Err(_) => break,
            }
            i += 1;
        }

        Ok(file)
    }
    pub fn write_all(&self) -> Result<(), exempi2::Error> {
        let mut file = match XmpFile::new_from_file(&self.path, exempi2::OpenFlags::FOR_UPDATE) {
            Ok(file) => file,
            Err(err) => return Err(err),
        };
        
        let mut xmp = Xmp::new();
        let mut flags = PropFlags::default();
        flags.insert(PropFlags::VALUE_IS_ARRAY);
        flags.insert(PropFlags::ARRAY_IS_UNORDERED);
        
        for tag in &self.tags {
            xmp.append_array_item(DUBLIN_CORE_SCHEMA, "dc:subject", flags, tag, PropFlags::default());
        }

        if self.date.is_some() { // should we really overwrite dates? I mean if you wanna correct
                                 // the original date you should be able to no?
            let _ = xmp.set_property(DUBLIN_CORE_SCHEMA, "dc:created", &self.date.unwrap().to_string(), PropFlags::default());
            let _ = xmp.set_property(EXIF_SCHEMA, "exif:DateTimeOriginal", &self.date.unwrap().to_string(), PropFlags::default());
            let _ = xmp.set_property(XMP_SCHEMA, "xmp:CreateDate", &self.date.unwrap().to_string(), PropFlags::default());
        }
        //println!("{}", xmp.serialize(SerialFlags::empty(), 2).unwrap());
        file.put_xmp(&xmp).unwrap(); //TODO don't unwrap
        let _ = file.close(exempi2::CloseFlags::SAFE_UPDATE);
        Ok(())
    }

    pub fn write_tags(&self) -> Result<(), exempi2::Error> {
        let mut file = match XmpFile::new_from_file(&self.path, exempi2::OpenFlags::FOR_UPDATE) {
            Ok(file) => file,
            Err(err) => return Err(err),
        };

        let mut xmp = Xmp::new();
        let mut flags = PropFlags::default();
        flags.insert(PropFlags::VALUE_IS_ARRAY);
        flags.insert(PropFlags::ARRAY_IS_UNORDERED);

        for tag in &self.tags {
            xmp.append_array_item(DUBLIN_CORE_SCHEMA, "dc:subject", flags, tag, PropFlags::default());
        }

        file.put_xmp(&xmp);
        file.close(exempi2::CloseFlags::SAFE_UPDATE);
        Ok(())
    }
    pub fn write_created_date(&self) -> Result<(), exempi2::Error> {
        let mut file = match XmpFile::new_from_file(&self.path, exempi2::OpenFlags::FOR_UPDATE) {
            Ok(file) => file,
            Err(err) => return Err(err),
        };

        let mut xmp = Xmp::new();
        
        if self.date.is_some() { // should we really overwrite dates? I mean if you wanna correct
                                 // the original date you should be able to no?
            xmp.set_property(DUBLIN_CORE_SCHEMA, "dc:created", &self.date.unwrap().to_string(), PropFlags::default());
            xmp.set_property(EXIF_SCHEMA, "exif:DateTimeOriginal", &self.date.unwrap().to_string(), PropFlags::default());
            xmp.set_property(XMP_SCHEMA, "xmp:CreateDate", &self.date.unwrap().to_string(), PropFlags::default());
        }

        file.put_xmp(&xmp);
        file.close(exempi2::CloseFlags::SAFE_UPDATE);
        Ok(())
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
            //dbg!(file);
        }
        Err(_) => {}
    }
    match File::read(PathBuf::from("./testing/test2.jpg")) {
        Ok(mut file) => {
            file.date = Some(DateTime::parse_from_rfc3339("2025-02-06T23:23:23+01:00").unwrap());
            file.tags = vec![
                "test".to_string(),
                "ULTRAKILL".to_string()
            ];
            file.write_all().unwrap();
            dbg!(file);
        }
        Err(_) => {}
    }
    //benchmark!({ File::read(PathBuf::from("./testing/test.jpg")) }, 10000);
    //benchmark!(println!("test"), 1000);
}
