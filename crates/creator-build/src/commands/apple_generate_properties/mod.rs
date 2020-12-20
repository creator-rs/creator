use super::Command;
use crate::{error::Result, types::*};
use std::fs::File;
use std::path::PathBuf;

pub struct AppleGenerateProperties {
    pub out_dir: PathBuf,
    pub properties: InfoPlist,
    pub binary: bool,
}

impl Command for AppleGenerateProperties {
    type Deps = ();
    type Output = ();

    fn run(&self) -> Result<Self::Output> {
        // Create Info.plist file
        let file_path = self.out_dir.join("Info.plist");
        let file = File::create(file_path)?;
        // Write to Info.plist file
        match self.binary {
            true => plist::to_writer_binary(file, &self.properties)?,
            false => plist::to_writer_xml(file, &self.properties)?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_run() {
        let dir = tempfile::tempdir().unwrap();
        let cmd = AppleGenerateProperties {
            out_dir: dir.path().to_owned(),
            properties: InfoPlist {
                categorization: Categorization {
                    bundle_package_type: None,
                    application_category_type: Some(AppCategoryType::Business),
                },
                ..Default::default()
            },
            binary: false,
        };
        cmd.run().unwrap();
        let file_path = dir.path().join("Info.plist");
        let result = std::fs::read_to_string(&file_path).unwrap();
        println!("{}", result);
        let properties: InfoPlist = plist::from_file(&file_path).unwrap();
        println!("{:?}", properties);
    }
}

// <?xml version=\"1.0\" encoding="UTF-8"?>
// <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
// <plist version=\"1.0\">
// <dict>
// 	<key>CFBundleDevelopmentRegion</key>
// 	<string>${DEVELOPMENT_LANGUAGE}</string>
// 	<key>CFBundleExecutable</key>
// 	<string>${PROJECT_NAME}</string>
// 	<key>CFBundleIdentifier</key>
// 	<string>${APP_BUNDLE_IDENTIFIER}</string>
// 	<key>CFBundleInfoDictionaryVersion</key>
// 	<string>6.0</string>
// 	<key>CFBundleName</key>
// 	<string>${PROJECT_NAME}</string>
// 	<key>CFBundlePackageType</key>
// 	<string>APPL</string>
// 	<key>CFBundleShortVersionString</key>
// 	<string>1.0</string>
// 	<key>CFBundleVersion</key>
// 	<string>1</string>
// 	<key>UILaunchStoryboardName</key>
// 	<string>LaunchScreen</string>
// 	<key>UIRequiresFullScreen</key>
// 	<false/>
// 	<key>UISupportedInterfaceOrientations</key>
// 	<array>
// 		<string>UIInterfaceOrientationPortrait</string>
// 		<string>UIInterfaceOrientationLandscapeLeft</string>
// 		<string>UIInterfaceOrientationLandscapeRight</string>
// 		<string>UIInterfaceOrientationPortraitUpsideDown</string>
// 	</array>
// </dict>
// </plist>
