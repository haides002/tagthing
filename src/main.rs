mod file;
#[macro_use]
mod utils;
mod tagcache;


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::file::File;
    use crate::tagcache::TagCache;

    #[test]
    fn benchmark_cache_search() {
        benchmark!(
            {
                let mut test = File::read(PathBuf::from("./testing/test.jpg")).unwrap();
                test.add_tag("ULTRAKILL");
                let cache = TagCache::new(&vec![test]);
                dbg!(cache.search("TRA"));
            },
            100
        );
    }
}
