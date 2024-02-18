pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn get_entries() -> Vec<String> {
    let efi = efivar::system();
    let res_entries = efi.get_boot_entries();
    let entries = match res_entries {
        Ok(res) => res.collect(),
        Err(_) => vec![]
    };
    entries.iter().map(|(res, varname)| {
        let entry = res.as_ref().ok().and_then(|it| Some(&it.entry));
        format!("{varname}: {:?}", entry)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
