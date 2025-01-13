use std::env;

pub fn pwd(physical: bool) -> String {
    let logical_path = env::current_dir();
    let path = if physical {
        logical_path.and_then(|path| path.canonicalize())
    } else {
        logical_path
    };

    match path {
        Ok(path) => path.to_string_lossy().into_owned(),
        Err(e) => {
            panic!("Error: {}", e);
        }
    }
}
