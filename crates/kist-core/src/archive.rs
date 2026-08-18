use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

        assert!((summary.savings_percent() - 35.0).abs() < f32::EPSILON);
    }
}
