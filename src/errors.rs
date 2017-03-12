use libcratesio;

error_chain! {
    foreign_links {
        CratesIO(libcratesio::Error);
    }
}
