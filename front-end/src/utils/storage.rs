use web_sys::window;

pub fn set_local_storage(key: &str, value: &str) {
    if let Some(storage) = window()
        .expect("window not available")
        .local_storage()
        .expect("local storage not available")
    {
        storage
            .set_item(key, value)
            .expect("set item in localStorage");
    }
}

// Function to get a value from localStorage 
pub fn get_local_storage(key: &str) -> Option<String> {
    if let Some(storage) = window()
        .expect("window not available")
        .local_storage()
        .expect("local storage not available")
    {
        match storage.get_item(key).ok().flatten() {
            None => {
                return None;
            }
            Some(value) => {
                if value.is_empty() {
                    return None;
                } else {
                    let return_value: Option<String> = Some(value.to_string());
                    return return_value;
                }
            }
        }
    }
    None
}
