/// stringify
///
/// Remap a slice of string slices to a vector of owned strings.
pub fn stringify(strs: &[&str]) -> Vec<String> {
    strs.iter().map(ToString::to_string).collect()
}
