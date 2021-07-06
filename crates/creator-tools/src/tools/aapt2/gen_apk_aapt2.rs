use crate::error::*;
use crate::tools::*;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use android_manifest::*;


pub fn aapt2_compile(
    sdk: &AndroidSdk,
    project_path: &Path,
    input: &Path,
    output_directory: &Path,
    build_dir: &Path,
    package_label: String,
) -> Result<PathBuf> {
    if !build_dir.exists() {
        create_dir_all(&build_dir)?;
    }
    let apk_path = build_dir.join(format!("{}-unaligned.apk", package_label));
    let mut aapt2_compile = sdk.build_tool(bin!("aapt2"), Some(project_path))?;
    aapt2_compile
        .arg("compile")
        .arg(input)
        .arg("-o")
        .arg(output_directory);
    aapt2_compile.output_err(true)?;
    Ok(apk_path)
}

pub fn aapt2_link(
    sdk: &AndroidSdk,
    project_path: &Path,
    manifest_path: &Path,
    output_apk: &Path,
    flat_file: &Path,
    build_dir: &Path,
    assets: Option<PathBuf>,
    res: Option<PathBuf>,
    package_label: String,
    target_sdk_version: u32,
) -> Result<PathBuf> {
    let apk_path = build_dir.join(format!("{}-unaligned.apk", package_label));
    let mut aapt2_link = sdk.build_tool(bin!("aapt2"), Some(project_path))?;
    aapt2_link
        .arg("link")
        .arg("-o")
        .arg(output_apk)
        .arg("-I")
        .arg(sdk.android_jar(target_sdk_version)?)
        .arg("--manifest")
        .arg(manifest_path)
        .arg("-R")
        .arg(flat_file)
        .arg("--java")
        .arg(project_path)
        .arg("--auto-add-overlay");
    if let Some(assets) = &assets {
        aapt2_link
            .arg("--proto-format")
            .arg(dunce::simplified(assets));
    }
    aapt2_link.output_err(true)?;
    Ok(apk_path)
}

// pub fn zipalign_apk(
//     sdk: &AndroidSdk,
//     unaligned_apk_path: &Path,
//     package: &str,
//     build_dir: &Path,
// ) -> Result<PathBuf> {
//     let unsigned_apk_path = build_dir.join(format!("{}.apk", package));
//     let mut zipalign = sdk.build_tool(bin!("zipalign"), None)?;
//     // Usage: zipalign [-f] [-p] [-v] [-z] <align> infile.zip outfile.zip
//     zipalign
//         .arg("-f")
//         .arg("-p")
//         .arg("-v")
//         .arg("z")
//         .arg(unaligned_apk_path)
//         .arg(&unsigned_apk_path);
//     zipalign.output_err(true)?;
//     Ok(unsigned_apk_path)
// }

#[cfg(test)]
mod tests {
    use crate::commands::android;

    use super::*;

    #[test]
    fn builder_test() {
        let sdk = AndroidSdk::from_env().unwrap();
        let package_label = "test";
        let target_sdk_version = 30;
        let manifest = android::gen_minimal_android_manifest(
            &package_label.to_string(),
            None,
            "0.0.1".to_string(),
            None,
            None,
            target_sdk_version,
            None,
            None,);
            let manifest_path = android::save_android_manifest(Path::new("D:\\programing\\work\\creator-rs\\creator\\examples\\3d\\res\\android\\mipmap-xxhdpi\\"), &manifest).unwrap();
            assert!(manifest_path.exists());
        let _aapt2_link = aapt2_link(
            &sdk,
            Path::new("D:\\programing\\work\\creator-rs\\creator\\examples\\3d\\res\\android\\mipmap-xxhdpi\\"), 
            &manifest_path,
            Path::new("D:\\programing\\work\\creator-rs\\creator\\examples\\3d\\res\\android\\mipmap-xxhdpi\\com.afwsamples.testdpc_7.0.2-7002_minAPI21(nodpi)_apkmirror.com.apk"), 
            Path::new("D:\\programing\\work\\creator-rs\\creator\\examples\\3d\\res\\android\\mipmap-xxhdpi\\mipmap-xxhdpi_ic_launcher.png.flat"), 
            Path::new("D:\\programing\\work\\creator-rs\\creator\\examples\\3d\\res\\android\\mipmap-xxhdpi\\"), 
            None, 
            None, 
            package_label.to_string(),
            target_sdk_version).unwrap();
    }
}

