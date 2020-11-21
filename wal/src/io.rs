use once_cell::sync::Lazy;
use parking_lot::Mutex;
use snafu::{ResultExt, Snafu};
use std::fs::File;

#[derive(Debug, Snafu)]
pub enum IoError {
    FailedToWriteDataUnix { source: nix::Error },
    FailedToWriteDataOther { source: std::io::Error },
    FailedToCloneFile { source: std::io::Error },
    FailedToSeek { source: std::io::Error },
}

#[cfg(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "linux",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub fn write(
    file: &File,
    header_bytes: &[u8],
    data_bytes: &[u8],
    offset: u64,
) -> Result<(), IoError> {
    use nix::sys::uio::{pwritev, IoVec};
    use std::os::unix::io::AsRawFd;

    let iovec = [
        IoVec::from_slice(header_bytes),
        IoVec::from_slice(data_bytes),
    ];

    pwritev(file.as_raw_fd(), &iovec, offset as i64).context(FailedToWriteDataUnix)?;

    Ok(())
}

#[cfg(not(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "linux",
    target_os = "netbsd",
    target_os = "openbsd"
)))]
static MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

#[cfg(not(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "linux",
    target_os = "netbsd",
    target_os = "openbsd",
)))]
pub fn write(
    file: &File,
    header_bytes: &[u8],
    data_bytes: &[u8],
    offset: u64,
) -> Result<(), IoError> {
    use std::io::{Seek, SeekFrom, Write};

    let _ = MUTEX.lock();
    let mut file = file.try_clone().context(FailedToCloneFile)?;
    file.seek(SeekFrom::Start(offset)).context(FailedToSeek)?;
    file.write_all(header_bytes)
        .context(FailedToWriteDataOther)?;
    file.write_all(data_bytes).context(FailedToWriteDataOther)?;
    Ok(())
}
