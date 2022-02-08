
#[derive(clap::Args, Debug, Clone)]
pub struct CyoobArgs {
    /// the filename of the qbloq to convert
    pub filename: String,

    /// the name to put for the author
    pub object_name: String,
    /// the name to put for the author
    pub author: String,
    /// the description text
    pub description: String,

    #[clap(long = "coverImage")]
    /// the cover image to add to the cyoob
    pub cover_image: Option<String>,
    /// whether or not the qbloq has debris objects
    #[clap(long = "hasDebris")]
    pub has_debris: bool,
    /// whether or not the qbloq has bomb objects
    #[clap(long = "hasBomb")]
    pub has_bomb: bool,
    /// whether or not the qbloq should show basegame arrows
    #[clap(long = "showArrows")]
    pub show_arrows: bool,
}