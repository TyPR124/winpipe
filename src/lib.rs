//! A tiny library wrapping Windows Pipes API

// #![warn(missing_docs)]

#[cfg(test)]
mod tests;

use winapi::um::{
    namedpipeapi::{CreatePipe, /* DisconnectNamedPipe, */ PeekNamedPipe},
    fileapi::{ReadFile, FlushFileBuffers, WriteFile},
    handleapi::{CloseHandle, DuplicateHandle},
    processthreadsapi::GetCurrentProcess,
    winnt::{HANDLE, DUPLICATE_SAME_ACCESS},
};

use std::{
    ptr,
    io::{self, Read, Write}
};

pub struct Sender {
    handle: HANDLE,
}

pub struct Receiver {
    handle: HANDLE,
}

impl Drop for Sender {
    fn drop(&mut self) {
        let r = unsafe { CloseHandle(self.handle) };
        debug_assert_ne!(0, r);
    }
}

impl Drop for Receiver {
    fn drop(&mut self) {
        let r = unsafe { CloseHandle(self.handle) };
        debug_assert_ne!(0, r);
    }
}

impl Sender {
    // pub fn try_clone(&self) -> Result<Self, winerr::Error> {
    //     let mut new_handle = ptr::null_mut();
    //     let r = unsafe {
    //         // Process handle
    //         let ph = GetCurrentProcess();
    //         DuplicateHandle(ph, self.handle, ph, &mut new_handle, 0, 0, DUPLICATE_SAME_ACCESS)
    //     };
    //     if r == 0 {
    //         Err(winerr::last_error())?;
    //     }
    //     Ok(Self { handle: new_handle })
    // }
    pub fn as_handle(&self) -> &HANDLE {
        &self.handle
    }
    pub unsafe fn as_handle_mut(&mut self) -> &mut HANDLE {
        &mut self.handle
    }
}

impl Receiver {
    // pub fn try_clone(&self) -> Result<Self, winerr::Error> {
    //     let mut new_handle = ptr::null_mut();
    //     let r = unsafe {
    //         // Process handle
    //         let ph = GetCurrentProcess();
    //         DuplicateHandle(ph, self.handle, ph, &mut new_handle, 0, 0, DUPLICATE_SAME_ACCESS)
    //     };
    //     if r == 0 {
    //         Err(winerr::last_error())?;
    //     }
    //     Ok(Self { handle: new_handle })
    // }
    pub fn as_handle(&self) -> &HANDLE {
        &self.handle
    }
    pub unsafe fn as_handle_mut(&mut self) -> &mut HANDLE {
        &mut self.handle
    }
    pub fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut read = 0;
        let r = unsafe {
            PeekNamedPipe(self.handle, buf.as_mut_ptr().cast(), buf.len() as u32, &mut read, &mut 0, &mut 0)
        };
        if r == 0 {
            Err(winerr::last_error())?;
        }
        Ok(read as usize)
    }
    pub fn available_bytes(&self) -> u32 {
        let mut avail = 0;
        let r = unsafe {
            PeekNamedPipe(self.handle, ptr::null_mut(), 0, ptr::null_mut(), &mut avail, ptr::null_mut())
        };
        debug_assert_ne!(0, r);
        avail
    }
}

impl Write for Sender {
    fn flush(&mut self) -> io::Result<()> {
        let r = unsafe {
            FlushFileBuffers(self.handle)
        };
        if r == 0 {
            Err(winerr::last_error())?;
        }
        Ok(())
    }
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut written = 0;
        let r = unsafe {
            WriteFile(self.handle, buf.as_ptr().cast(), buf.len() as u32, &mut written, ptr::null_mut())
        };
        if r == 0 {
            Err(winerr::last_error())?;
        }
        Ok(written as usize)
    }
}

impl Read for Receiver {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut read = 0;
        let r = unsafe {
            ReadFile(self.handle, buf.as_mut_ptr().cast(), buf.len() as u32, &mut read, ptr::null_mut())
        };
        if r == 0 {
            Err(winerr::last_error())?;
        }
        Ok(read as usize)
    }
}

// impl Sender {
//     pub fn disconnect(self) -> io::Result<()> {
//         let r = unsafe {
//             DisconnectNamedPipe(self.handle)
//         };
//         if r == 0 {
//             Err(winerr::last_error())?;
//         }
//         Ok(())
//     }
// }

// impl Receiver {
//     pub fn disconnect(self) -> io::Result<()> {
//         let r = unsafe {
//             DisconnectNamedPipe(self.handle)
//         };
//         if r == 0 {
//             Err(winerr::last_error())?;
//         }
//         Ok(())
//     }
// }

pub fn unnamed() -> Result<(Sender, Receiver), winerr::Error> {
    let mut tx: HANDLE = ptr::null_mut();
    let mut rx: HANDLE = ptr::null_mut();
    let r = unsafe {
        CreatePipe(&mut rx, &mut tx, ptr::null_mut(), 0)
    };
    if r == 0 {
        Err(winerr::last_error())?;
    }
    let tx = Sender { handle: tx };
    let rx = Receiver { handle: rx };
    Ok((tx, rx))
}