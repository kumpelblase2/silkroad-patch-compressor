use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use xz2::stream::{LzmaOptions, Stream};

const LZMA_DICT_SIZE: u32 = 33554432;
const HEADER_SIZE_OFFSET: usize = 10;

fn fix_header_size(original_size: u64, data: &mut Vec<u8>) {
    let size_bytes = original_size.to_le_bytes();
    for i in (0..5).rev() {
        data.insert(0, size_bytes[i]);
    }

    // We also have to fix the size in the LZMA header because for some reason it is set to `-1` even
    // though we give it a fixed size buffer ...
    for i in 0..8 {
        data[HEADER_SIZE_OFFSET + i] = size_bytes[i];
    }
}

fn compress<T: Read>(data: T) -> Vec<u8> {
    let lzma_stream = Stream::new_lzma_encoder(
        LzmaOptions::new_preset(6)
            .expect("Should be able to use lzma preset.")
            .dict_size(LZMA_DICT_SIZE),
    )
    .expect("Should be able to create lzma instance.");
    let mut encoder = xz2::read::XzEncoder::new_stream(data, lzma_stream);
    let mut output = Vec::new();
    let written = encoder
        .read_to_end(&mut output)
        .expect("Should be able to read encoded data.");
    assert!(written > 0);
    output
}

fn decompress<T: Read + Seek>(mut data: T) -> Vec<u8> {
    data.seek(SeekFrom::Start(5))
        .expect("Should be able to skip additional header");
    let mut decoder = xz2::read::XzDecoder::new_stream(
        data,
        Stream::new_lzma_decoder(u64::MAX).expect("Should be able to create decoder"),
    );
    let mut output = Vec::new();
    let read = decoder.read_to_end(&mut output);
    let read = read.expect("Should be able to finish file");
    assert!(read > 0);
    output
}

enum Mode {
    Compress,
    Decompress,
}

fn main() {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() < 2 || args.len() > 3 {
        println!("Usage: xz2-compress <input> <output>");
        return;
    }

    let mode = if args[0] == "-d" {
        args.remove(0);
        Mode::Decompress
    } else {
        Mode::Compress
    };

    let input = args.remove(0);
    let output = args.remove(0);

    let input_data = File::open(input).expect("Could not open input file.");
    let mut output_data = File::options()
        .write(true)
        .create(true)
        .open(output)
        .expect("Could not create output file.");

    match mode {
        Mode::Compress => {
            let original_size = input_data
                .metadata()
                .expect("Should be able to get original file size.")
                .len();
            let mut compressed = compress(input_data);
            fix_header_size(original_size, &mut compressed);
            output_data
                .write_all(&compressed)
                .expect("Should be able to write compressed data.");
        }
        Mode::Decompress => {
            let decompressed = decompress(input_data);
            output_data
                .write_all(&decompressed)
                .expect("Should be able to write decompressed data.");
        }
    }
}
