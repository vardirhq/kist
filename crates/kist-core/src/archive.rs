use serde::{Deserialize, Serialize};
use std::{fs::File, path::{Path, PathBuf}};
use thiserror::Error;
use zip::ZipArchive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArchiveFormat {
    Zip,
    SevenZip,
    Tar,
    TarGz,
    TarZstd,
}

impl ArchiveFormat {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Zip => "ZIP",
            Self::SevenZip => "7z",
            Self::Tar => "TAR",
            Self::TarGz => "TAR.GZ",
            Self::TarZstd => "TAR.ZST",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionLevel {
    Fast,
    #[default]
    Balanced,
    Maximum,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveEntry {
    pub path: PathBuf,
    pub size: u64,
    pub compressed_size: Option<u64>,
    pub is_directory: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveSummary {
    pub path: PathBuf,
    pub format: ArchiveFormat,
    pub entries: usize,
    pub original_size: u64,
    pub compressed_size: u64,
}

impl ArchiveSummary {
    pub fn savings_percent(&self) -> f32 {
        if self.original_size == 0 {
            return 0.0;
        }

        (1.0 - self.compressed_size as f32 / self.original_size as f32) * 100.0
    }
}

#[derive(Debug, Clone)]
pub struct ArchiveDocument {
    pub summary: ArchiveSummary,
    pub entries: Vec<ArchiveEntry>,
}

#[derive(Debug, Error)]
pub enum ArchiveError {
    #[error("unsupported archive format: {0}")]
    UnsupportedFormat(String),
    #[error("could not open archive: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid or damaged ZIP archive: {0}")]
    Zip(#[from] zip::result::ZipError),
}

pub fn open_archive(path: impl AsRef<Path>) -> Result<ArchiveDocument, ArchiveError> {
    let path = path.as_ref();
    let format = detect_format(path)?;

    match format {
        ArchiveFormat::Zip => open_zip(path),
        _ => Err(ArchiveError::UnsupportedFormat(format.label().to_owned())),
    }
}

fn detect_format(path: &Path) -> Result<ArchiveFormat, ArchiveError> {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    if name.ends_with(".zip") {
        Ok(ArchiveFormat::Zip)
    } else if name.ends_with(".7z") {
        Ok(ArchiveFormat::SevenZip)
    } else if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
        Ok(ArchiveFormat::TarGz)
    } else if name.ends_with(".tar.zst") || name.ends_with(".tzst") {
        Ok(ArchiveFormat::TarZstd)
    } else if name.ends_with(".tar") {
        Ok(ArchiveFormat::Tar)
    } else {
        Err(ArchiveError::UnsupportedFormat(
            path.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("unknown")
                .to_owned(),
        ))
    }
}

fn open_zip(path: &Path) -> Result<ArchiveDocument, ArchiveError> {
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut entries = Vec::with_capacity(archive.len());
    let mut original_size = 0_u64;
    let mut compressed_size = 0_u64;

    for index in 0..archive.len() {
        let file = archive.by_index(index)?;
        let size = file.size();
        let packed = file.compressed_size();

        original_size = original_size.saturating_add(size);
        compressed_size = compressed_size.saturating_add(packed);
        entries.push(ArchiveEntry {
            path: PathBuf::from(file.name()),
            size,
            compressed_size: Some(packed),
            is_directory: file.is_dir(),
        });
    }

    Ok(ArchiveDocument {
        summary: ArchiveSummary {
            path: path.to_path_buf(),
            format: ArchiveFormat::Zip,
            entries: entries.len(),
            original_size,
            compressed_size,
        },
        entries,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_archive_savings() {
        let summary = ArchiveSummary {
            path: "example.7z".into(),
            format: ArchiveFormat::SevenZip,
            entries: 4,
            original_size: 1_000,
            compressed_size: 650,
        };

        assert!((summary.savings_percent() - 35.0).abs() < 0.001);
    }

    #[test]
    fn detects_supported_extensions() {
        assert_eq!(detect_format(Path::new("photos.zip")).unwrap(), ArchiveFormat::Zip);
        assert_eq!(detect_format(Path::new("backup.tar.gz")).unwrap(), ArchiveFormat::TarGz);
        assert_eq!(detect_format(Path::new("backup.tar.zst")).unwrap(), ArchiveFormat::TarZstd);
    }
}
