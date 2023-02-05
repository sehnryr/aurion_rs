use log::{debug, error};

/// Get view state from an html content.
/// The view state is used to send a request to the server.
pub fn get_view_state<T: AsRef<str>>(text: T) -> Option<String> {
    let text = text.as_ref();
    let splitter = "name=\"javax.faces.ViewState\"";
    let splitted = text.split(splitter).collect::<Vec<&str>>();
    if splitted.len() < 2 {
        error!("Failed to get view state.");
        return None;
    }
    let view_state = splitted[1].split("value=\"").collect::<Vec<&str>>()[1]
        .split("\"")
        .collect::<Vec<&str>>()[0];
    debug!("View state: {}", view_state);
    Some(view_state.to_string())
}
