use chrono::{DateTime, FixedOffset};
use exempi2::{PropFlags, Xmp, XmpFile};
use std::path::PathBuf;

use crate::utils::parse_date;

const EXIF_SCHEMA: &str = "http://ns.adobe.com/exif/1.0/";
const DUBLIN_CORE_SCHEMA: &str = "http://purl.org/dc/elements/1.1/";
const XMP_SCHEMA: &str = "http://ns.adobe.com/xap/1.0/";

/// File wrapper storing the path, created date and tags
#[derive(Debug, Clone, Hash)]
pub struct File {
    pub path: PathBuf,
    pub date: Option<chrono::DateTime<FixedOffset>>,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl File {
    /// Reads file from specified path, returns a file or the Error
    pub fn read(path: PathBuf) -> Result<File, exempi2::Error> {
        let file = match XmpFile::new_from_file(&path, exempi2::OpenFlags::ONLY_XMP) {
            Ok(file) => file,
            Err(err) => return Err(err),
        };
        let xmp = match file.get_new_xmp() {
            Ok(xmp) => xmp,
            Err(err) => return Err(err),
        };
        //println!(
        //    "{}\n\n\n",
        //    xmp.serialize(exempi2::SerialFlags::empty(), 2)
        //        .unwrap()
        //        .to_str()
        //        .unwrap()
        //);

        let date: Option<DateTime<chrono::FixedOffset>> = {
            if let Ok(date) =
                xmp.get_property(XMP_SCHEMA, "xmp:CreateDate", &mut PropFlags::empty())
            {
                if let Some(parsed) = parse_date(date.to_str().unwrap_or_default()) {
                    Some(parsed)
                } else {
                    None
                }
            } else if let Ok(date) = xmp.get_property(
                EXIF_SCHEMA,
                "exif:DateTimeOriginal",
                &mut PropFlags::empty(),
            ) {
                if let Some(parsed) = parse_date(date.to_str().unwrap_or_default()) {
                    Some(parsed)
                } else {
                    None
                }
            } else if let Ok(date) =
                xmp.get_property(DUBLIN_CORE_SCHEMA, "dc:created", &mut PropFlags::empty())
            {
                if let Some(parsed) = parse_date(date.to_str().unwrap_or_default()) {
                    Some(parsed)
                } else {
                    None
                }
            } else {
                None
            }
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
                Ok(tag) => file.tags.push(tag.to_str().unwrap().into()),
                Err(_) => break,
            }
            i += 1;
        }

        Ok(file)
    }

    /// Recursively reads all files from the specified directory
    pub fn read_dir(path: PathBuf) -> Vec<Self> {
        let mut files: Vec<Self> = Vec::new();
        for dir_entry in std::fs::read_dir(path).expect("Path specified can not be read") {
            if let Ok(entry) = dir_entry {
                if entry.path().is_dir() {
                    files.append(&mut File::read_dir(entry.path()));
                } else {
                    if let Ok(file) = File::read(entry.path()) {
                        files.push(file);
                    }
                }
            }
        }
        files
    }

    pub fn write_all(&self) -> Result<(), exempi2::Error> {
        self.write_tags()?;
        self.write_created_date()?;
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
            let _ = xmp.append_array_item(
                DUBLIN_CORE_SCHEMA,
                "dc:subject",
                flags,
                tag,
                PropFlags::default(),
            );
        }

        let _ = file.put_xmp(&xmp);
        let _ = file.close(exempi2::CloseFlags::SAFE_UPDATE);
        Ok(())
    }
    pub fn write_created_date(&self) -> Result<(), exempi2::Error> {
        let mut file = match XmpFile::new_from_file(&self.path, exempi2::OpenFlags::FOR_UPDATE) {
            Ok(file) => file,
            Err(err) => return Err(err),
        };

        let mut xmp = Xmp::new();

        if self.date.is_some() {
            // should we really overwrite dates? I mean if you wanna correct
            // the original date you should be able to no?
            let _ = xmp.set_property(
                DUBLIN_CORE_SCHEMA,
                "dc:created",
                &self.date.unwrap().to_string(),
                PropFlags::default(),
            );
            let _ = xmp.set_property(
                EXIF_SCHEMA,
                "exif:DateTimeOriginal",
                &self.date.unwrap().to_string(),
                PropFlags::default(),
            );
            let _ = xmp.set_property(
                XMP_SCHEMA,
                "xmp:CreateDate",
                &self.date.unwrap().to_string(),
                PropFlags::default(),
            );
        }

        let _ = file.put_xmp(&xmp);
        let _ = file.close(exempi2::CloseFlags::SAFE_UPDATE);
        Ok(())
    }

    pub fn add_tag(&mut self, new_tag: &str) {
        self.tags.push(new_tag.to_string());
    }

    pub fn set_tags(&mut self, new_tags: Vec<String>) {
        self.tags = new_tags;
    }

    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub fn remove_tag(&mut self, tag_to_remove: &str) -> Result<(), ()> {
        match self
            .tags
            .iter()
            .position(|x| -> bool { tag_to_remove == x })
        {
            Some(index) => {
                self.tags.remove(index);
                Ok(())
            }
            None => Err(()),
        }
    }
}
