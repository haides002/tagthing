use std::path::PathBuf;

use tagcache::TagCache;
use utils::parse_date;

mod file;
#[macro_use]
mod utils;
mod tagcache;

fn main() {
    println!("Hello, world!");
    use crate::file::File;
    let mut test = File::read_dir(PathBuf::from("/home/linus/downloads/amogus/"));
    dbg!(&test);
    let cache = TagCache::new(&test);
    dbg!(cache);

    dbg!(parse_date("2022-03-02T22:37:46"));
}

#[cfg(test)]
mod tests {
    use crate::file::File;
    use crate::tagcache::TagCache;
    use std::path::PathBuf;

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
