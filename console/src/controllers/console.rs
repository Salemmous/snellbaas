use actix_files::{Files, NamedFile};

pub fn get_service() -> Files {
    Files::new("/", "./web/build")
        .index_file("index.html")
        .default_handler(
            NamedFile::open("./web/build/index.html").expect("Unable to load SPA index.html"),
        )
}
