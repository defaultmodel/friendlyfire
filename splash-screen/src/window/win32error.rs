// Stolen from here :
// https://github.com/Rust-WinGUI/win32-error/blob/master/src/lib.rs

use std::fmt;

use windows::{
    Win32::{
        Foundation::{GetLastError, WIN32_ERROR},
        System::Diagnostics::Debug::{
            FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM,
            FORMAT_MESSAGE_IGNORE_INSERTS, FormatMessageW,
        },
    },
    core::PWSTR,
};

const UNKNOWN_ERROR_TEXT: &str = "Unknown error";

/// A generic Error around the error codes cause by `GetLastError`
///
/// # Usage
///
/// ```
///    return Err(Win32Error::from_last_error());
/// ```
#[derive(Debug)]
pub struct Win32Error {
    // Error code returned by GetLastError
    error_code: u32,

    // Message returned by FormatMessage
    description: Option<String>,
}

impl Win32Error {
    /// Creates an error by calling GetLastError
    pub fn from_last_error() -> Self {
        let error_code = unsafe { GetLastError() };
        Self::from_error(error_code)
    }

    /// Creates an error from the error struct given by `GestLastError`
    fn from_error(error: WIN32_ERROR) -> Self {
        unsafe {
            let error_code = error.0;
            let description: PWSTR = PWSTR::null();

            // Should be zero or num of chars copied
            let chars_copied = FormatMessageW(
                FORMAT_MESSAGE_IGNORE_INSERTS
                    | FORMAT_MESSAGE_FROM_SYSTEM
                    | FORMAT_MESSAGE_ALLOCATE_BUFFER,
                None,
                error_code,
                0,
                description,
                0,
                None,
            );

            // Very likely wrong err number was passed, and no message exists
            if chars_copied == 0 {
                return Win32Error {
                    error_code,
                    description: None,
                };
            }

            Win32Error {
                error_code,
                description: description.to_string().ok(),
            }
        }
    }
}

impl fmt::Display for Win32Error {
    /// Prints an error description in the following format:
    /// **Error code**: **Error message**, eg. 5: Access denied
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.description.as_ref() {
            Some(s) => format!("{}: {}", self.error_code, s),
            None => format!("{}: {}", self.error_code, UNKNOWN_ERROR_TEXT),
        }
        .fmt(f)
    }
}

impl std::error::Error for Win32Error {}
