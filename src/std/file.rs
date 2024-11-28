pub enum FileOpen {
    Read,
}

pub struct File {
    fd: libc::c_int,
}

impl File {
    pub fn open(path: &core::ffi::CStr, option: FileOpen) -> Result<Self, ()> {
        let ptr = path.as_ptr();

        // Check file exists
        #[cfg(feature = "stdout")]
        {
            let mut stat: libc::stat = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
            let fs = unsafe { libc::stat(ptr, (&mut stat) as *mut libc::stat) };

            if fs < 0 {
                crate::println!("file {path:?} did not exist on stat {fs:?}");
                return Err(());
            }
        }

        let oflag = match option {
            FileOpen::Read => libc::O_RDONLY,
        };

        let fd = unsafe { libc::open(ptr, oflag) };

        if fd < 0 {
            crate::println!("open {path:?} returned non-zero exit code {fd:?}");
            return Err(());
        }

        Ok(Self { fd })
    }

    pub fn read<const READ_BUFFER_SIZE: usize>(
        &self,
        buf: &mut [u8; READ_BUFFER_SIZE],
    ) -> Result<usize, ()> {
        let res = unsafe {
            libc::read(
                self.fd,
                buf.as_mut_ptr() as *mut core::ffi::c_void,
                READ_BUFFER_SIZE,
            )
        };

        if res < 0 {
            crate::println!("read returned non-zero exit code {res:?}");
            return Err(());
        }

        Ok(res as usize)
    }
}
