use text_colorizer::Colorize;

pub fn colorize(color: bool, path: String) -> String {
    if !color {
        return path;
    }

    let colorful_path = path
        .chars()
        .enumerate()
        .map(|(i, c)| match i % 7 {
            0 => c.to_string().red().to_string(),
            1 => c.to_string().bright_red().to_string(),
            2 => c.to_string().yellow().to_string(),
            3 => c.to_string().green().to_string(),
            4 => c.to_string().bright_blue().to_string(),
            5 => c.to_string().blue().to_string(),
            6 => c.to_string().purple().to_string(),
            _ => c.to_string().black().to_string(),
        })
        .collect::<String>();

    colorful_path
}
