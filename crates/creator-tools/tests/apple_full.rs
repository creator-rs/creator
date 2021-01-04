#![cfg(target_os = "macos")]

use creator_tools::types::*;
use creator_tools::*;

fn get_minimal_info_plist(name: &str) -> InfoPlist {
    InfoPlist {
        localization: Localization {
            bundle_development_region: Some("en".to_owned()),
            ..Default::default()
        },
        launch: Launch {
            bundle_executable: Some(name.to_owned()),
            ..Default::default()
        },
        identification: Identification {
            bundle_identifier: "com.test.test-id".to_owned(),
            ..Default::default()
        },
        bundle_version: BundleVersion {
            bundle_version: Some("1".to_owned()),
            bundle_info_dictionary_version: Some("1.0".to_owned()),
            bundle_short_version_string: Some("1.0".to_owned()),
            ..Default::default()
        },
        naming: Naming {
            bundle_name: Some(name.to_owned()),
            ..Default::default()
        },
        categorization: Categorization {
            bundle_package_type: Some("APPL".to_owned()),
            ..Default::default()
        },
        launch_interface: LaunchInterface {
            launch_storyboard_name: Some("LaunchScreen".to_owned()),
            ..Default::default()
        },
        styling: Styling {
            requires_full_screen: Some(false),
            ..Default::default()
        },
        orientation: Orientation {
            supported_interface_orientations: Some(vec![
                InterfaceOrientation::Portrait,
                InterfaceOrientation::PortraitUpsideDown,
                InterfaceOrientation::LandscapeLeft,
                InterfaceOrientation::LandscapeRight,
            ]),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[test]
fn test_apple_full() {
    let tempdir = tempfile::tempdir().unwrap();
    let dir = tempdir.path();
    let name = gen_minimal_project(dir).unwrap();

    // Create target dir
    let target_dir = dir.join("target");
    std::fs::create_dir(&target_dir).unwrap();

    // Generate app folder
    let app_dir = gen_apple_app(&target_dir, &name, None, None).unwrap();
    assert!(app_dir.exists());

    // Compile app
    let build_target = AppleTarget::X86_64AppleIos;
    let profile = Profile::Release;
    apple_rust_compile(&name, build_target, dir, profile, vec![]).unwrap();
    let out_dir = dir
        .join("target")
        .join(build_target.rust_triple())
        .join(&profile);

    // Copy binary to app folder
    let bin_path = out_dir.join(&name);
    std::fs::copy(&bin_path, &app_dir.join(&name)).unwrap();

    // Generate Info.plist
    let properties = get_minimal_info_plist(&name);
    gen_apple_plist(&app_dir, &properties, false).unwrap();

    // Sign bundle
    codesign(&app_dir, true, None, None).unwrap();

    // Install and launch on simulator
    let device = launch_apple_app(
        &app_dir,
        "iPhone 8",
        &properties.identification.bundle_identifier,
        false,
    )
    .unwrap();
    device.shutdown().unwrap();
}