use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use zip::CompressionMethod;
use zip::write::FileOptions;
use zip::write::ZipWriter;


fn main() {
    let files = vec![
        "data/big_image.jpg",
        "data/big_image.png",
        "data/example.png",
        "data/lua54.dll",
        "data/small_image.jpg",
        "data/small_image.png",
        "data/some_text.txt",
    ];

    let arc_opt = FileOptions::default().compression_method(CompressionMethod::Deflated);

    let cursor = Cursor::new(Vec::new());
    let mut archive = ZipWriter::new(cursor);

    files.iter().for_each(|&file| {
        archive.start_file(file, arc_opt.clone()).unwrap();

        let mut file_buf: Vec<u8> = Vec::new();

        File::open(file).unwrap().read_to_end(&mut file_buf).unwrap();

        archive.write(&file_buf).unwrap();
    });

    let data = archive.finish().unwrap();

    let archive_path = Path::new("file.zip");

    File::create(&archive_path).unwrap().write_all(data.into_inner().as_ref()).unwrap();
}
