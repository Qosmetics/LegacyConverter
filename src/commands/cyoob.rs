use zip::{ZipWriter, write::FileOptions, CompressionMethod};
use std::io::{Cursor, Write, Read};
use std::path::Path;

use crate::args::cyoob::CyoobArgs;
use crate::data::cyoob::CyoobConfig;
use crate::data::package::{PackageJson, Descriptor};

pub fn convert(args: CyoobArgs) {
    let file_path= Path::new(&args.filename);
    if !file_path.exists() {
        panic!("Provided file did not exist, please pass a valid filename")
    }
    
    let file_stem = file_path.file_stem().unwrap().to_string_lossy();
    let cyoob_file_name = format!("{}.cyoob", file_stem);

    let android_file_name = file_path.file_name().unwrap().to_string_lossy().to_string();

    let descriptor = Descriptor{
            object_name: args.object_name, 
            author: args.author, 
            description: args.description
        };
        
    let config = CyoobConfig {
        has_debris: args.has_debris,
        has_slider: false,
        has_bomb: args.has_bomb,
        show_arrows: args.show_arrows,
        is_mirrorable: false, 
        is_legacy: Some(true)
    };

    // if I implement what the warning suggests here it can't deduce type and it's cringe
    #[allow(clippy::redundant_closure)]
    let cover_image_path= args.cover_image.as_ref().map(|cover_image| Path::new(cover_image));

    let package = PackageJson {
        android_file_name: android_file_name.clone(),
        pc_file_name: "".to_string(),
        cover_image: cover_image_path.map(|inner| inner.file_name().unwrap().to_string_lossy().to_string()),
        descriptor,
        config: serde_json::to_value(config).unwrap(),
    };

    let package_string = serde_json::to_string_pretty(&package).unwrap();

    let cyoob_file = std::fs::File::create(cyoob_file_name).unwrap();
    let mut cyoob = ZipWriter::new(cyoob_file);

    let options = FileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(0o755);

    cyoob.start_file(android_file_name, options).unwrap();
    let mut qbloq_file = std::fs::File::open(file_path).unwrap();
    let mut qbloq_buffer = Cursor::new(Vec::new());
    qbloq_file.read_to_end(qbloq_buffer.get_mut()).unwrap();
    cyoob.write_all(qbloq_buffer.get_mut()).unwrap();

    if let Some (cover_image_file_name) = cover_image_path.map(|inner| inner.file_name().unwrap().to_string_lossy().to_string()) {
        cyoob.start_file(cover_image_file_name, options).unwrap();
        let mut cover_file = std::fs::File::open(file_path).unwrap();
        let mut cover_buffer = Cursor::new(Vec::new());
        cover_file.read_to_end(cover_buffer.get_mut()).unwrap();
        cyoob.write_all(cover_buffer.get_mut()).unwrap();
    }

    cyoob.start_file("package.json", options).unwrap();
    cyoob.write_all(package_string.as_bytes()).unwrap();

    cyoob.finish().unwrap();
}