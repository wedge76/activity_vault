use windows::{
    w,
    Win32::{
        Foundation::HWND,
        UI::{
            Accessibility::{SetWinEventHook, HWINEVENTHOOK},
            WindowsAndMessaging::{
                MessageBoxW, EVENT_SYSTEM_FOREGROUND, MB_OK, WINEVENT_OUTOFCONTEXT,GetWindowTextW
            },
        },
    },
};

fn main() {
    let hook = unsafe {
        SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_SYSTEM_FOREGROUND,
            None,
            Some(win_event_hook_callback),
            0,
            0, 
            WINEVENT_OUTOFCONTEXT,
        )
    };
    // Make sure the hook is installed; a real application would want to do more
    // elaborate error handling
    assert!(!hook.is_invalid(), "Failed to install hook");

    // Have the system spin up a message loop (and get a convenient way to exit
    // the application for free)
    let _ = unsafe {
        MessageBoxW(
            None,
            w!("Click OK to terminate"),
            w!("Event hook running"),
            MB_OK,
        )
    };
}

unsafe extern "system" fn win_event_hook_callback(
    _hook_handle: HWINEVENTHOOK,
    _event_id: u32,
    _window_handle: HWND,
    _object_id: i32,
    _child_id: i32,
    _thread_id: u32,
    _timestamp: u32,
) {
    let result = get_window_title(_window_handle);    
    
    match result{
        Ok(title) => println!("Event received: title: {}", title),
        Err(_) => ()
    }    
}

fn get_window_title(hwnd: HWND) -> Result<String, ()> {
    let title: String;
    unsafe {
        let mut v: Vec<u16> = vec![0; 255];
        let title_len = GetWindowTextW(hwnd, &mut v);
        title = String::from_utf16_lossy(&v[0..(title_len as usize)]);        
    };    

    Ok(title)
}