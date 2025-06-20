use std::ffi::CStr;
use std::mem;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use libc::{
    IN_CLOSE_WRITE, IN_CREATE, IN_DELETE, IN_MODIFY, inotify_add_watch, inotify_init, read,
};

const BUF_LEN: usize = 1024;

#[repr(C)]
struct InotifyEvent {
    wd: i32,
    mask: u32,
    cookie: u32,
    len: u32,
}

pub fn watch_loop(path: &Path) -> std::io::Result<()> {
    let fd = unsafe { inotify_init() };
    if fd < 0 {
        return Err(std::io::Error::last_os_error());
    }

    let c_path = std::ffi::CString::new(path.as_os_str().as_bytes()).unwrap();
    let wd = unsafe {
        inotify_add_watch(
            fd,
            c_path.as_ptr(),
            IN_CREATE | IN_MODIFY | IN_DELETE | IN_CLOSE_WRITE,
        )
    };
    if wd < 0 {
        return Err(std::io::Error::last_os_error());
    }

    let mut buffer = [0u8; BUF_LEN];

    println!("[oxsync] Watching for changes in {:?}", path);

    loop {
        let len = unsafe { read(fd, buffer.as_mut_ptr() as *mut _, BUF_LEN) };
        if len < 0 {
            return Err(std::io::Error::last_os_error());
        }

        let mut offset = 0;
        while offset < len as usize {
            let event = unsafe { &*(buffer.as_ptr().add(offset) as *const InotifyEvent) };
            let name_ptr = unsafe { buffer.as_ptr().add(offset + mem::size_of::<InotifyEvent>()) };
            let name = unsafe {
                CStr::from_ptr(name_ptr as *const _)
                    .to_string_lossy()
                    .into_owned()
            };
            if event.mask & IN_CREATE != 0 {
                println!("Created: {name}");
            }
            if event.mask & IN_MODIFY != 0 {
                println!("Modified: {name}");
            }
            if event.mask & IN_CLOSE_WRITE != 0 {
                println!("Written: {name}");
            }
            if event.mask & IN_DELETE != 0 {
                println!("Deleted: {name}");
            }

            offset += mem::size_of::<InotifyEvent>() + event.len as usize;
        }
    }
}
