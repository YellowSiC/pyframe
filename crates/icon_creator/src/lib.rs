// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use anyhow::Result;
use image::imageops::FilterType;
use std::path::Path;

/// Creates an ICO file with multiple icon sizes from a source image.
///
/// # Arguments
/// * `source_path` - Path to the source image file.
/// * `target_path` - Path to the output ICO file.
///
/// # Returns
/// * `Result<()>` - Ok if successful, or an error.
pub fn create_ico_impl(source_path: &Path, target_path: &Path) -> Result<()> {
    // Load the input image
    let img = image::open(source_path)?;

    // Create a new icon directory
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

    // Generate icons in multiple sizes
    for size in &[16, 24, 32, 48, 64, 128, 256] {
        let resized = img.resize_exact(*size, *size, FilterType::Lanczos3);
        let rgba_data = resized.to_rgba8().to_vec();

        let icon_img = ico::IconImage::from_rgba_data(*size, *size, rgba_data);
        let icon_entry = ico::IconDirEntry::encode(&icon_img)?;

        icon_dir.add_entry(icon_entry);
    }

    // Write the generated icon file to the target path
    let target_file = std::fs::File::create(target_path)?;
    icon_dir.write(target_file)?;

    println!("Icon generated at {:?}", target_path);
    Ok(())
}
