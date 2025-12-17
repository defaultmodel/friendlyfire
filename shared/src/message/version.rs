use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Semantic Versioning 2.0.0 used to detect incompability between different components of the program.
/// Stored as integers to avoid string-based comparisons at runtime.
///
/// Follows `MAJOR.MINOR.PATCH` semantics.
/// See https://semver.org/
#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    /// Gets increment when there are breaking changes in the protocol.
    /// Implies no backward compatibility with other `major` versions.
    major: u8,
    /// Gets incremented when there are backward compatible changes in the protocol.
    /// Imoplies backward compatibility with other `minor` versions.
    minor: u8,
    /// Gets incremented when there are backward compatible bug fixes.
    patch: u8,
}

impl Version {
    fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

/// Errors that can occur while parsing a semantic version string.
#[derive(Debug, PartialEq, Eq)]
pub enum VersionParseError {
    /// The string does not match the `MAJOR.MINOR.PATCH` format.
    InvalidFormat,
    /// One of the version components failed to parse as a number.
    InvalidNumber,
}

impl FromStr for Version {
    type Err = VersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');

        let major = parts.next().ok_or(VersionParseError::InvalidFormat)?;
        let minor = parts.next().ok_or(VersionParseError::InvalidFormat)?;
        let patch = parts.next().ok_or(VersionParseError::InvalidFormat)?;

        if parts.next().is_some() {
            return Err(VersionParseError::InvalidFormat);
        }

        Ok(Version {
            major: major
                .parse()
                .map_err(|_| VersionParseError::InvalidNumber)?,
            minor: minor
                .parse()
                .map_err(|_| VersionParseError::InvalidNumber)?,
            patch: patch
                .parse()
                .map_err(|_| VersionParseError::InvalidNumber)?,
        })
    }
}

impl TryFrom<String> for Version {
    type Error = VersionParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&str> for Version {
    type Error = VersionParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
