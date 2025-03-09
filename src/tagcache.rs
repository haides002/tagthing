use crate::file::File;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct TagCache(Vec<String>);
impl TagCache {
    /// Create new TagCache from given Vec of files
    pub fn new(files: &Vec<File>) -> Self {
        let mut cache: Vec<String> = Vec::new();
        for file in files {
            cache.append(&mut file.get_tags().clone());
        }

        cache.sort();
        cache.dedup();

        TagCache(cache)
    }

    /// Case insensitive fuzzy search.
    pub fn search(&self, query: &str) -> Vec<String> {
        self.0
            .iter()
            .filter(|x| -> bool { x.to_lowercase().find(&query.to_lowercase()).is_some() })
            .map(|x: &String| -> String { x.clone() })
            .collect::<Vec<String>>()
    }
}
