pub(crate) mod models;

/// Check if the current directory is an node project
/// by checking if package.json exists
pub(crate) fn is_node_project() -> bool {
    std::fs::metadata("package.json").is_ok()
}
