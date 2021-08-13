use clap::Clap;

#[derive(Clap)]
#[clap(name = "Chipper")]
/// Image in, images out
pub struct Opts {
    /// Source image
    pub path: String,

    /// Chip size
    #[clap(short, long, default_value = "544")]
    pub size: u32,

    /// Chip format
    #[clap(long, default_value = "jpg")]
    pub format: String,

    /// Output directory
    #[clap(short, long)]
    pub outdir: Option<String>,
}
