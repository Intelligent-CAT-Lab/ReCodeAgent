use std::fs;
use std::os::fd;
use std::io::Result;
#[cfg(target_os = "windows")]
pub const FS_OPEN_READ: &str = "rb";
#[cfg(target_os = "windows")]
pub const FS_OPEN_WRITE: &str = "wb";
#[cfg(target_os = "windows")]
pub const FS_OPEN_READWRITE: &str = "rwb";
#[cfg(not(target_os = "windows"))]
pub const FS_OPEN_READ: &str = "r";
#[cfg(not(target_os = "windows"))]
pub const FS_OPEN_WRITE: &str = "w";
#[cfg(not(target_os = "windows"))]
pub const FS_OPEN_READWRITE: &str = "rw";
pub fn fs_error(s: &str) {
    unimplemented!()
}
pub fn fs_open(path: &str, flags: &str) -> Option<fd::OwnedFd> {
    unimplemented!()
}
pub fn fs_close(fd: fd::OwnedFd) -> Result<()> {
    unimplemented!()
}
pub fn fs_rename(from: &str, to: &str) -> Result<()> {
    unimplemented!()
}
pub fn fs_stat(path: &str) -> Result<fs::Metadata> {
    unimplemented!()
}
pub fn fs_fstat(fd: &fd::OwnedFd) -> Result<fs::Metadata> {
    unimplemented!()
}
pub fn fs_lstat(path: &str) -> Result<fs::Metadata> {
    unimplemented!()
}
pub fn fs_ftruncate(fd: &fd::OwnedFd, length: u64) -> Result<()> {
    unimplemented!()
}
pub fn fs_truncate(path: &str, length: u64) -> Result<()> {
    unimplemented!()
}
pub fn fs_chown(path: &str, uid: u32, gid: u32) -> Result<()> {
    unimplemented!()
}
pub fn fs_fchown(fd: &fd::OwnedFd, uid: u32, gid: u32) -> Result<()> {
    unimplemented!()
}
pub fn fs_lchown(path: &str, uid: u32, gid: u32) -> Result<()> {
    unimplemented!()
}
pub fn fs_size(path: &str) -> Result<u64> {
    unimplemented!()
}
pub fn fs_fsize(fd: &fd::OwnedFd) -> Result<u64> {
    unimplemented!()
}
pub fn fs_read(path: &str) -> Result<Vec<u8>> {
    unimplemented!()
}
pub fn fs_nread(path: &str, len: u64) -> Result<Vec<u8>> {
    unimplemented!()
}
pub fn fs_fread(fd: &fd::OwnedFd) -> Result<Vec<u8>> {
    unimplemented!()
}
pub fn fs_fnread(fd: &fd::OwnedFd, len: u64) -> Result<Vec<u8>> {
    unimplemented!()
}
pub fn fs_write(path: &str, data: &[u8]) -> Result<u64> {
    unimplemented!()
}
pub fn fs_nwrite(path: &str, data: &[u8], len: u64) -> Result<u64> {
    unimplemented!()
}
pub fn fs_fwrite(fd: &fd::OwnedFd, data: &[u8]) -> Result<u64> {
    unimplemented!()
}
pub fn fs_fnwrite(fd: &fd::OwnedFd, data: &[u8], len: u64) -> Result<u64> {
    unimplemented!()
}
pub fn fs_mkdir(path: &str, mode: u32) -> Result<()> {
    unimplemented!()
}
pub fn fs_rmdir(path: &str) -> Result<()> {
    unimplemented!()
}
pub fn fs_exists(path: &str) -> bool {
    unimplemented!()
}