use windows::Win32::Foundation::{CloseHandle, HANDLE};

pub struct Handle(pub HANDLE);

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.0);
        }
    }
}
