#[macro_use]
extern crate failure;
use failure::Fallible;
use std::collections::BTreeMap;
use std::{
    env,
    fs::{copy, File},
    io::{BufWriter, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};
use serde_json;

use cargo_metadata::Metadata;

#[derive(Debug, Serialize, Deserialize)]
struct KaiosManifest {
    version: String,
    name: String,
    description: String,
    r#type: String,
    launch_path: String,
    icons: BTreeMap<String, String>,
    developer: BTreeMap<String, String>,
    locales: BTreeMap<String, BTreeMap<String, String>>,
    default_locale: String,
}

#[derive(Debug, Fail)]
enum GenKaiosManifestError {
    #[fail(display = "Couldn't find a root package")]
    NoMainPackageFound,
    #[fail(display = "Couldn't find the package author")]
    NoAuthorFound,
    #[fail(display = "A Key doesn't contain a string value")]
    KeyValueStringError,
    #[fail(display = "No default locale found")]
    NoDefaultLocales,
}

pub fn main() -> Fallible<()> {
    let metadata = package_metadata();
    let manifest = generate_kaios_manifest(package_metadata_to_kaios_manifest(metadata)?);
    copy(manifest, "./static/manifest.webapp")?; // Copy file to cargo web static folder
    Ok(())
}

fn package_metadata() -> Metadata {
    let mut cmd = cargo_metadata::MetadataCommand::new();
    cmd.exec().unwrap()
}

fn package_metadata_to_kaios_manifest(
    metadata: Metadata,
) -> Result<KaiosManifest, GenKaiosManifestError> {
    // Search workspace packages for kaios package infos
    let mut package_with_kaios_metadata = metadata
        .workspace_members
        .iter()
        .filter_map(|pkg_id| {
            let package = metadata
                .packages
                .iter()
                .find(|&pkg| pkg.id == *pkg_id)
                .unwrap();
            match &package.metadata {
                serde_json::Value::Object(p) => match p.get("kaios") {
                    Some(_) => Some(package),
                    _ => None,
                },
                serde_json::Value::Null => None,
                _ => None,
            }
        })
        .nth(0);
    let kaios_metadata = match package_with_kaios_metadata {
        Some(p) => p.metadata.get("kaios"),
        _ => {
            // We couldn't find specific kaios metadata informations
            // Get the first workspace package to at least get some infos
            package_with_kaios_metadata = metadata
                .workspace_members
                .iter()
                .filter_map(|pkg_id| metadata.packages.iter().find(|&pkg| pkg.id == *pkg_id))
                .nth(0);
            None
        }
    };
    let package_with_kaios_metadata = match package_with_kaios_metadata {
        Some(p) => p,
        None => return Err(GenKaiosManifestError::NoMainPackageFound),
    };

    //         println!("{:#?}", kaios_metadata);
    let name = package_with_kaios_metadata.name.clone();
    let description = package_with_kaios_metadata
        .description
        .as_ref()
        .unwrap_or(&"".to_owned())
        .clone();
    let version = &package_with_kaios_metadata.version;
    let version = format!("{}.{}.{}", version.major, version.minor, version.patch).to_owned();
    let developer = match package_with_kaios_metadata.authors.iter().nth(0) {
        Some(a) => a,
        None => return Err(GenKaiosManifestError::NoAuthorFound),
    };
    let developer: BTreeMap<String, String> = [("name".to_owned(), developer.clone())]
        .iter()
        .cloned()
        .collect();
    let kaios_type = kaios_metadata
        .unwrap_or(&serde_json::Value::Null)
        .get("app-type")
        .map_or(None, |t| t.as_str())
        .unwrap_or("web")
        .to_owned();
    let launch_path = kaios_metadata
        .unwrap_or(&serde_json::Value::Null)
        .get("app-launch-path")
        .map_or(None, |t| t.as_str())
        .unwrap_or("")
        .to_owned();

    let empty_tmp_map = serde_json::map::Map::new();
    let icons_tmp = kaios_metadata
        .unwrap_or(&serde_json::Value::Null)
        .get("app-icons")
        .map_or(None, |t| t.as_object())
        .unwrap_or(&empty_tmp_map);
    let mut icons = BTreeMap::new();
    for (key, v) in icons_tmp.iter() {
        match v.as_str() {
            Some(v) => {
                let _ = icons.insert(key.clone(), v.to_owned());
            }
            None => return Err(GenKaiosManifestError::KeyValueStringError),
        }
    }

    let empty_tmp_map = serde_json::map::Map::new();
    let mut default_locale: Option<String> = None;
    let locales_tmp = kaios_metadata
        .unwrap_or(&serde_json::Value::Null)
        .get("app-locales")
        .map_or(None, |t| t.as_object())
        .unwrap_or(&empty_tmp_map);
    let mut locales = BTreeMap::new();
    for (key, v) in locales_tmp.iter() {
        match v.as_object() {
            Some(v) => {
                default_locale = match default_locale {
                    Some(l) => Some(l),
                    None => Some(key.clone()),
                };
                let mut locale = BTreeMap::new();
                for (lk, lv) in v.iter() {
                    match lv.as_str() {
                        Some(lv) => {
                            let _ = locale.insert(lk.clone(), lv.to_owned());
                        }
                        None => return Err(GenKaiosManifestError::KeyValueStringError),
                    }
                }
                let _ = locales.insert(key.clone(), locale);
            }
            None => return Err(GenKaiosManifestError::KeyValueStringError),
        }
    }
    let default_locale = match default_locale {
        Some(l) => l,
        None => return Err(GenKaiosManifestError::NoDefaultLocales),
    };

    Ok(KaiosManifest {
        name: name,
        description: description,
        version: version,
        r#type: kaios_type,
        launch_path: launch_path,
        icons: icons,
        developer: developer,
        locales: locales,
        default_locale: default_locale,
    })
}

fn generate_kaios_manifest(manifest: KaiosManifest) -> std::path::PathBuf {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("manifest.webapp");
    let mut file = BufWriter::new(File::create(&path).unwrap());
    writeln!(
        &mut file,
        "{}",
        serde_json::to_string_pretty(&manifest).unwrap()
    )
    .unwrap();
    path
}
