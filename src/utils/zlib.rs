use flate2::read::ZlibDecoder;
use std::io::Read;

pub fn compress(data: Vec<u8>) -> Vec<u8> {
    use libz_sys::{compress2, compressBound, uLong, uLongf, Z_BEST_SPEED};
    let src_len = data.len() as uLong;
    let mut dst_len = unsafe { compressBound(src_len) };
    let mut dst = vec![0u8; dst_len as usize];
    let status = unsafe {
        compress2(
            dst.as_mut_ptr(),
            &mut dst_len as *mut uLongf,
            data.as_ptr(),
            src_len,
            Z_BEST_SPEED,
        )
    };
    if status != 0 {
        panic!("Zlib compression failed with status: {}", status);
    }
    dst.truncate(dst_len as usize);
    dst
}

pub fn decompress(data: Vec<u8>) -> Vec<u8> {
    let mut decoder = ZlibDecoder::new(&data[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).unwrap();
    decompressed_data
}
