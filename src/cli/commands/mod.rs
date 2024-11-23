pub(crate) mod check_upgrade;

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// Check if you can upgrade the package in your project
    CheckDep {
        /// The name of the npm package to check
        package_name: String,
        /// Target Version to check compatibility with
        target_version: String,
        /// Glob patterns to ignore certain dependencies
        #[clap(short = 'i', long = "ignore")]
        ignore: Vec<String>,
    },
}
