use log::{debug, error};

/// Get the form id from an html content.
/// The form id is used to send ajax requests to get the menu.
pub fn get_form_id<T: AsRef<str>>(text: T) -> Option<u8> {
    let text = text.as_ref();
    let splitter = "chargerSousMenu = function() {PrimeFaces.ab({s:\"form:j_idt";
    let splitted = text.split(splitter).collect::<Vec<&str>>();
    if splitted.len() < 2 {
        error!("Failed to get form id.");
        return None;
    }
    let form_id = splitted[1].split("\"").collect::<Vec<&str>>()[0];
    debug!("Form id: {}", form_id);
    Some(form_id.parse().unwrap())
}

/// Get the schedule form id from an html content.
/// The schedule form id is used to send ajax requests to get the schedule.
pub fn get_schedule_form_id<T: AsRef<str>>(text: T) -> Option<u8> {
    let text = text.as_ref();
    let splitter = "\" class=\"schedule\"";
    let splitted = text.split_once(splitter);
    if splitted.is_none() {
        error!("Failed to get schedule form id.");
        return None;
    }
    let splitted = splitted.unwrap();
    let schedule_form_id = splitted.0.rsplit_once("id=\"form:j_idt").unwrap().1;
    debug!("Schedule form id: {}", schedule_form_id);
    Some(schedule_form_id.parse().unwrap())
}
