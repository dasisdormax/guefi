use super::System;

pub struct SystemImpl;

impl System for SystemImpl {
    async fn get_boot_entries() -> Result<Vec<String>, String> {
        let efi = efivar::system();
        let res_entries = efi.get_boot_entries();
        let entries = res_entries.map_err(|err| {
            format!("Error getting boot entries: {:?}.", err)
        })?;
        let entry_names = entries.map(|(res, varname)| {
            let entry = res.as_ref().ok().and_then(|it| Some(&it.entry));
            format!("{varname}: {:?}", entry)
        }).collect();
        Ok(entry_names)
    }
}