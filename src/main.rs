use affix::*;
// Testing Terminate wl display
use std::ptr::NonNull;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::{Arc, Mutex};

static DISPLAY_PTR: AtomicPtr<std::ffi::c_void> = AtomicPtr::new(std::ptr::null_mut());

#[repr(transparent)]
pub struct DisplayHandle(NonNull<affix::wl_display>);
unsafe impl Send for DisplayHandle {}

impl DisplayHandle {
    // Helper function
    pub fn as_ptr(&self) -> *mut affix::wl_display {
        self.0.as_ptr()
    }

    pub fn new(ptr: *mut affix::wl_display) -> Option<Self> {
        NonNull::new(ptr).map(DisplayHandle)
    }
}

impl Drop for DisplayHandle {
    fn drop(&mut self) {
        unsafe {
            wl_display_destroy(self.as_ptr());
        }
    }
}

fn main() {
    unsafe {
        // Server
        let raw_display = wl_display_create();
        if raw_display.is_null() {
            println!("Failed to create Wayland display");
            return;
        }

        let display = DisplayHandle::new(raw_display).expect("Failed to create DisplayHandle");
        let display_arc = Arc::new(Mutex::new(Some(display)));

        // Socket
        let socket = wl_display_add_socket_auto(raw_display);
        if socket.is_null() {
            println!("Failed to create Wayland socket");
            wl_display_destroy(raw_display);
            return;
        }
        println!("Created Wayland socket");

        DISPLAY_PTR.store(raw_display as *mut std::ffi::c_void, Ordering::SeqCst);

        let display_for_handler = Arc::clone(&display_arc);
        ctrlc::set_handler(move || {
            println!("\n Received interrupt signal, shutting down....");

            let mut guard = display_for_handler.lock().unwrap();
            if let Some(display) = guard.take() {
                wl_display_terminate(display.as_ptr());
            }
        })
        .expect("Failed to set interrupt handler");

        println!("Wayland server running. Press Ctrl+C to stop");
        wl_display_run(raw_display);

        // clean up
        {
            let mut guard = display_arc.lock().unwrap();
            if let Some(display) = guard.take() {
                wl_display_destroy(display.as_ptr());
            }
        }
        println!("Wayland server stopped");
    }
}

#[cfg(test)]
mod tests {
    use affix::*;

    #[test]
    fn test_connect_and_disconnect() {
        unsafe {
            let display = wl_display_connect(std::ptr::null());

            if display.is_null() {
                println!("No Wayland server available, skipping tests");
            } else {
                println!("Connected to Wayland server");
                let result = wl_display_roundtrip(display);
                assert!(result >= 0, "roundtrip failed");

                wl_display_disconnect(display);
            }
        }
    }
}
