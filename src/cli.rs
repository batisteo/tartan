use clap::Parser;
use regex::Regex;

#[derive(Debug)]
pub struct Parameters {
    pub filename: String,
    pub width: u32,
    pub height: u32,
    pub skip: usize,
    pub ugly: bool,
    pub pattern: String,
}

impl Parameters {
    pub fn get() -> Self {
        let args = Args::parse();
        let (width, height) = Self::dimensions(&args.size);

        Self {
            filename: args.filename,
            width,
            height,
            skip: args.skip,
            ugly: args.ugly,
            pattern: args.pattern,
        }
    }

    fn dimensions(size: &str) -> (u32, u32) {
        let wxh: Regex = Regex::new(r"^(?P<width>[0-9]+)[:x×-](?P<height>[0-9]+)$").unwrap();

        if size.is_empty() {
            (400, 400)
        } else {
            if let Some(size) = size.parse::<u32>().ok() {
                (size, size)
            } else if let Some(caps) = wxh.captures(&size) {
                (
                    caps["width"].parse::<u32>().unwrap(),
                    caps["height"].parse::<u32>().unwrap(),
                )
            } else {
                println!("Couldn’t understand the size, using default 400px");
                (400, 400)
            }
        }
    }
}

#[derive(Parser, Debug)]
struct Args {
    /// Output name
    #[clap(short = 'o', long, default_value = "tartan.png")]
    filename: String,

    /// Size of the produced image
    #[clap(short, long, default_value = "400")]
    size: String,

    /// Skip n colour spans in the pattern.
    #[clap(short = 'k', long, default_value_t = 0)]
    skip: usize,

    /// Faster render without noise and blur
    #[clap(short, long)]
    ugly: bool,

    /// Thread count
    #[clap(default_value = "W6T36G20T4RB20T4RB20T4G20T36R6")]
    pattern: String,

    #[clap(short = 'R', long, help_heading = "COLOURS")]
    red: Option<String>,

    #[clap(short = 'O', long, help_heading = "COLOURS")]
    orange: Option<String>,

    #[clap(short = 'Y', long, help_heading = "COLOURS")]
    yellow: Option<String>,

    #[clap(short = 'G', long, help_heading = "COLOURS")]
    green: Option<String>,

    #[clap(short = 'B', long, help_heading = "COLOURS")]
    blue: Option<String>,

    #[clap(short = 'P', long, help_heading = "COLOURS")]
    purple: Option<String>,

    #[clap(short = 'W', long, help_heading = "COLOURS")]
    white: Option<String>,

    #[clap(short = 'N', long, help_heading = "COLOURS")]
    grey: Option<String>,

    #[clap(short = 'T', long, help_heading = "COLOURS")]
    brown: Option<String>,

    #[clap(short = 'K', long, help_heading = "COLOURS")]
    black: Option<String>,
}
