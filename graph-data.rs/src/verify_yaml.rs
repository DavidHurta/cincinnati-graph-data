use cincinnati::plugins::internal::openshift_secondary_metadata_parser::plugin;
use cincinnati::plugins::internal::openshift_secondary_metadata_parser::plugin::graph_data_model::{BlockedEdge, Channel};
use regex::Regex;
use semver::Version;
use std::collections::HashSet;
use std::path::PathBuf;
use anyhow::Result as Fallible;

pub async fn run() -> Fallible<HashSet<Version>> {
    let data_dir = PathBuf::from(".");
    println!(
        "Looking for metadata in {:?}",
        data_dir.canonicalize()?
    );
    let all_files_regex = Regex::new(".*")?;
    let disallowed_errors: HashSet<plugin::DeserializeDirectoryFilesErrorDiscriminants> = [
        plugin::DeserializeDirectoryFilesErrorDiscriminants::File,
        plugin::DeserializeDirectoryFilesErrorDiscriminants::InvalidExtension,
        plugin::DeserializeDirectoryFilesErrorDiscriminants::MissingExtension,
        plugin::DeserializeDirectoryFilesErrorDiscriminants::Deserialize,
    ]
    .iter()
    .cloned()
    .collect();
    // Collect a list of mentioned versions
    let mut found_versions: HashSet<Version> = HashSet::new();

    println!("Verifying blocked edge files are valid");
    let blocked_edge_path = data_dir.join(plugin::BLOCKED_EDGES_DIR).canonicalize()?;
    let blocked_edge_vec = plugin::deserialize_directory_files::<BlockedEdge>(
        &blocked_edge_path,
        all_files_regex.clone(),
        &disallowed_errors,
    )
    .await?;
    for v in blocked_edge_vec.iter() {
        found_versions.insert(v.to.clone());
    }

    println!("Verifying channel files are valid");
    let channel_path = data_dir.join(plugin::CHANNELS_DIR).canonicalize()?;
    let channels_vec = plugin::deserialize_directory_files::<Channel>(
        &channel_path,
        all_files_regex.clone(),
        &disallowed_errors,
    )
    .await?;
    for c in channels_vec.iter() {
        for v in c.versions.iter() {
            found_versions.insert(v.clone());
        }
    }

    Ok(found_versions)
}
