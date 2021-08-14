use clap::Clap;

#[derive(Clap)]
#[clap(name = "Chipper")]
/// Image in, images out
pub struct Opts {
    /// Input image
    pub path: String,

    /// Output chip size
    #[clap(short, long, default_value = "544")]
    pub size: u32,

    /// Output chip format
    #[clap(long, default_value = "jpg")]
    pub format: String,

    /// Max input image size (GB)
    #[clap(long, default_value = "1")]
    pub mem: u8,

    /// Output directory
    #[clap(short, long)]
    pub outdir: Option<String>,
}

#[derive(Clap)]
#[clap(name = "Chipper GUI")]
/// Image in, GUI out
pub struct GuiOpts {
    /// Input image
    pub path: String,

    /// Max input image size (GB)
    #[clap(long, default_value = "1")]
    pub mem: u8,
}
