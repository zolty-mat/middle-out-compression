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
