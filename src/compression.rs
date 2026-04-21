use std::fs;
use std::io;
use std::path::Path;

/// Magic bytes identifying a MIDDLEOUT™ compressed file
pub const MAGIC: &[u8; 9] = b"MIDDLEOUT";
pub const VERSION: u8 = 1;
/// Header layout: MAGIC(9) + VERSION(1) + original_size(8) + padding_size(8) = 26 bytes
pub const HEADER_SIZE: usize = 26;

#[derive(Debug)]
pub struct MocHeader {
    pub original_size: u64,
    pub padding_size: u64,
}

impl MocHeader {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(HEADER_SIZE);
        bytes.extend_from_slice(MAGIC);
        bytes.push(VERSION);
        bytes.extend_from_slice(&self.original_size.to_le_bytes());
        bytes.extend_from_slice(&self.padding_size.to_le_bytes());
        bytes
    }

    pub fn from_bytes(data: &[u8]) -> io::Result<Self> {
        if data.len() < HEADER_SIZE {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "File too small to be a .moc file — or too big to be a coincidence"));
        }
        if &data[0..9] != MAGIC {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not a MIDDLEOUT™ file. Have you considered that your file isn't special enough?",
            ));
        }
        if data[9] != VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unsupported MIDDLEOUT™ version. Please upgrade to continue being disappointed.",
            ));
        }
        let original_size = u64::from_le_bytes(data[10..18].try_into().unwrap());
        let padding_size = u64::from_le_bytes(data[18..26].try_into().unwrap());
        Ok(MocHeader { original_size, padding_size })
    }
}

/// Compress a file using the proprietary MIDDLEOUT™ algorithm.
///
/// "Compression" here means: find the middle of the file and insert
/// padding equal to 10% of the original file size. The result is 10%+ larger.
/// This is working as intended.
pub fn compress(input_path: &Path, output_path: &Path) -> io::Result<(u64, u64)> {
    let data = fs::read(input_path)?;
    let original_size = data.len() as u64;

    // 10% padding — the "middle out" proprietary expansion
    let padding_size = (original_size / 10).max(1);
    let mid = data.len() / 2;

    // Generate extremely sophisticated padding
    let padding: Vec<u8> = (0..padding_size)
        .map(|_| rand::random::<u8>())
        .collect();

    let header = MocHeader { original_size, padding_size };

    let mut output = Vec::with_capacity(HEADER_SIZE + data.len() + padding_size as usize);
    output.extend_from_slice(&header.to_bytes());
    output.extend_from_slice(&data[..mid]);
    output.extend_from_slice(&padding);
    output.extend_from_slice(&data[mid..]);

    fs::write(output_path, &output)?;

    let compressed_size = output.len() as u64;
    Ok((original_size, compressed_size))
}

/// Decompress a MIDDLEOUT™ .moc file by locating and removing the middle padding.
///
/// Yes, we know what we did. We're removing what we added.
pub fn decompress(input_path: &Path, output_path: &Path) -> io::Result<u64> {
    let data = fs::read(input_path)?;
    let header = MocHeader::from_bytes(&data)?;

    let body = &data[HEADER_SIZE..];
    let total_expected = header.original_size + header.padding_size;

    if body.len() as u64 != total_expected {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "File body is {} bytes but expected {}. The middle may have shifted.",
                body.len(),
                total_expected
            ),
        ));
    }

    let mid = header.original_size as usize / 2;
    let padding = header.padding_size as usize;

    let mut restored = Vec::with_capacity(header.original_size as usize);
    restored.extend_from_slice(&body[..mid]);
    // skip the middle (that's the whole trick)
    restored.extend_from_slice(&body[mid + padding..]);

    fs::write(output_path, &restored)?;
    Ok(header.original_size)
}

pub fn read_header(path: &Path) -> io::Result<MocHeader> {
    let mut buf = vec![0u8; HEADER_SIZE];
    let data = fs::read(path)?;
    if data.len() < HEADER_SIZE {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "File is too small to contain a valid MIDDLEOUT™ header"));
    }
    buf.copy_from_slice(&data[..HEADER_SIZE]);
    MocHeader::from_bytes(&buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn write_temp(data: &[u8]) -> NamedTempFile {
        let f = NamedTempFile::new().unwrap();
        fs::write(f.path(), data).unwrap();
        f
    }

    // ── MocHeader serialization ──────────────────────────────────────────────

    #[test]
    fn header_round_trip() {
        let h = MocHeader { original_size: 12345, padding_size: 1234 };
        let bytes = h.to_bytes();
        assert_eq!(bytes.len(), HEADER_SIZE);
        let h2 = MocHeader::from_bytes(&bytes).unwrap();
        assert_eq!(h2.original_size, 12345);
        assert_eq!(h2.padding_size, 1234);
    }

    #[test]
    fn header_magic_is_correct() {
        let h = MocHeader { original_size: 0, padding_size: 0 };
        let bytes = h.to_bytes();
        assert_eq!(&bytes[0..9], b"MIDDLEOUT");
        assert_eq!(bytes[9], VERSION);
    }

    #[test]
    fn header_from_bytes_too_small() {
        let err = MocHeader::from_bytes(&[0u8; 10]).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn header_from_bytes_wrong_magic() {
        let mut bytes = MocHeader { original_size: 1, padding_size: 1 }.to_bytes();
        bytes[0] = b'X'; // corrupt magic
        let err = MocHeader::from_bytes(&bytes).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("Not a MIDDLEOUT"));
    }

    #[test]
    fn header_from_bytes_wrong_version() {
        let mut bytes = MocHeader { original_size: 1, padding_size: 1 }.to_bytes();
        bytes[9] = 99; // corrupt version
        let err = MocHeader::from_bytes(&bytes).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("version"));
    }

    // ── compress ────────────────────────────────────────────────────────────

    #[test]
    fn compress_output_is_larger_than_input() {
        let input = write_temp(b"Hello, MIDDLEOUT!");
        let output = NamedTempFile::new().unwrap();
        let (orig, compressed) = compress(input.path(), output.path()).unwrap();
        assert!(compressed > orig, "output ({compressed}) should be larger than input ({orig})");
    }

    #[test]
    fn compress_header_records_correct_original_size() {
        let data = b"Some test data for compression";
        let input = write_temp(data);
        let output = NamedTempFile::new().unwrap();
        compress(input.path(), output.path()).unwrap();
        let header = read_header(output.path()).unwrap();
        assert_eq!(header.original_size, data.len() as u64);
    }

    #[test]
    fn compress_padding_is_ten_percent_of_original() {
        let data = vec![0u8; 1000];
        let input = write_temp(&data);
        let output = NamedTempFile::new().unwrap();
        compress(input.path(), output.path()).unwrap();
        let header = read_header(output.path()).unwrap();
        assert_eq!(header.padding_size, 100, "padding should be 10% of 1000 bytes");
    }

    #[test]
    fn compress_output_size_equals_header_plus_original_plus_padding() {
        let data = vec![42u8; 200];
        let input = write_temp(&data);
        let output = NamedTempFile::new().unwrap();
        let (orig, compressed_size) = compress(input.path(), output.path()).unwrap();
        let expected = HEADER_SIZE as u64 + orig + (orig / 10).max(1);
        assert_eq!(compressed_size, expected);
    }

    // ── decompress ──────────────────────────────────────────────────────────

    #[test]
    fn decompress_restores_exact_bytes() {
        let original_data = b"The middle has been found.";
        let input = write_temp(original_data);
        let moc = NamedTempFile::new().unwrap();
        let restored = NamedTempFile::new().unwrap();

        compress(input.path(), moc.path()).unwrap();
        decompress(moc.path(), restored.path()).unwrap();

        let got = fs::read(restored.path()).unwrap();
        assert_eq!(got, original_data, "decompressed content must match original exactly");
    }

    #[test]
    fn decompress_returns_correct_original_size() {
        let data = vec![7u8; 500];
        let input = write_temp(&data);
        let moc = NamedTempFile::new().unwrap();
        let out = NamedTempFile::new().unwrap();
        compress(input.path(), moc.path()).unwrap();
        let size = decompress(moc.path(), out.path()).unwrap();
        assert_eq!(size, 500);
    }

    #[test]
    fn decompress_rejects_non_moc_file() {
        let garbage = write_temp(b"this is definitely not a .moc file, sorry");
        let out = NamedTempFile::new().unwrap();
        let err = decompress(garbage.path(), out.path()).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn decompress_rejects_truncated_body() {
        let data = vec![1u8; 100];
        let input = write_temp(&data);
        let moc = NamedTempFile::new().unwrap();
        compress(input.path(), moc.path()).unwrap();

        // Truncate body to simulate corruption
        let mut moc_bytes = fs::read(moc.path()).unwrap();
        moc_bytes.truncate(moc_bytes.len() - 5);
        let corrupt = write_temp(&moc_bytes);

        let out = NamedTempFile::new().unwrap();
        let err = decompress(corrupt.path(), out.path()).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("middle may have shifted"));
    }

    // ── round-trip edge cases ────────────────────────────────────────────────

    #[test]
    fn round_trip_empty_file() {
        let input = write_temp(b"");
        let moc = NamedTempFile::new().unwrap();
        let out = NamedTempFile::new().unwrap();
        compress(input.path(), moc.path()).unwrap();
        decompress(moc.path(), out.path()).unwrap();
        assert_eq!(fs::read(out.path()).unwrap(), b"");
    }

    #[test]
    fn round_trip_single_byte() {
        let input = write_temp(b"\xff");
        let moc = NamedTempFile::new().unwrap();
        let out = NamedTempFile::new().unwrap();
        compress(input.path(), moc.path()).unwrap();
        decompress(moc.path(), out.path()).unwrap();
        assert_eq!(fs::read(out.path()).unwrap(), b"\xff");
    }

    #[test]
    fn round_trip_odd_length() {
        let data = vec![0xABu8; 101]; // odd length
        let input = write_temp(&data);
        let moc = NamedTempFile::new().unwrap();
        let out = NamedTempFile::new().unwrap();
        compress(input.path(), moc.path()).unwrap();
        decompress(moc.path(), out.path()).unwrap();
        assert_eq!(fs::read(out.path()).unwrap(), data);
    }

    #[test]
    fn round_trip_even_length() {
        let data = vec![0xCDu8; 1024]; // even length
        let input = write_temp(&data);
        let moc = NamedTempFile::new().unwrap();
        let out = NamedTempFile::new().unwrap();
        compress(input.path(), moc.path()).unwrap();
        decompress(moc.path(), out.path()).unwrap();
        assert_eq!(fs::read(out.path()).unwrap(), data);
    }

    #[test]
    fn round_trip_binary_data() {
        let data: Vec<u8> = (0u8..=255).cycle().take(300).collect();
        let input = write_temp(&data);
        let moc = NamedTempFile::new().unwrap();
        let out = NamedTempFile::new().unwrap();
        compress(input.path(), moc.path()).unwrap();
        decompress(moc.path(), out.path()).unwrap();
        assert_eq!(fs::read(out.path()).unwrap(), data);
    }

    #[test]
    fn round_trip_large_file() {
        let data: Vec<u8> = (0u8..=255).cycle().take(100_000).collect();
        let input = write_temp(&data);
        let moc = NamedTempFile::new().unwrap();
        let out = NamedTempFile::new().unwrap();
        compress(input.path(), moc.path()).unwrap();
        decompress(moc.path(), out.path()).unwrap();
        assert_eq!(fs::read(out.path()).unwrap(), data);
    }

    // ── read_header ─────────────────────────────────────────────────────────

    #[test]
    fn read_header_matches_compressed_file() {
        let data = vec![99u8; 400];
        let input = write_temp(&data);
        let moc = NamedTempFile::new().unwrap();
        compress(input.path(), moc.path()).unwrap();

        let header = read_header(moc.path()).unwrap();
        assert_eq!(header.original_size, 400);
        assert_eq!(header.padding_size, 40); // 10% of 400
    }

    #[test]
    fn read_header_fails_on_tiny_file() {
        let tiny = write_temp(b"too small");
        let err = read_header(tiny.path()).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    }
}
