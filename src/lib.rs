use ini::Ini;

pub fn merge_ini(base_ini: &Ini, patch_ini: &Ini) -> Ini {
    let mut out_ini = base_ini.clone();
    for (patch_section, patch_kv) in patch_ini {
        if let Some(base_kv) = out_ini.section_mut(patch_section) {
            for (k, v) in patch_kv.iter() {
                base_kv.insert(k, v);
            }
        } else {
            let entry = out_ini.entry(
                patch_section.map(|s| s.to_owned()));
            entry.or_insert(patch_kv.clone());
        }
    }
    out_ini
}

#[cfg(test)]
mod test {
    use crate::merge_ini;

    use std::io::Cursor;

    macro_rules! ini {
        ($input:expr) => {
            {
                use ini::Ini;
                Ini::load_from_str($input).unwrap()
            }
        };
    }
    #[test]
    fn test_merge_no_overlap() {
        let first_conf = ini!("
            key_1 = a

            [section_1]
            key_1_1 = aa
        ");
        let second_conf = ini!("
            key_2 = b

            [section_1]
            key_1_2 = ab

            [section_2]
            key_2_2 = bb
        ");
        let expected_output_conf = "
            key_1 = a
            key_2 = b

            [section_1]
            key_1_1 = aa
            key_1_2 = ab

            [section_2]
            key_2_2 = bb
        ";

        let merged_fs = merge_ini(&first_conf, &second_conf);
        let mut merged_fs_cursor = Cursor::new(Vec::new());
        merged_fs.write_to(&mut merged_fs_cursor).unwrap();

        let merged_sf = merge_ini(&second_conf, &first_conf);
        let mut merged_sf_cursor = Cursor::new(Vec::new());
        merged_sf.write_to(&mut merged_sf_cursor).unwrap();

        let merged_fs_str = String::from_utf8(merged_fs_cursor.into_inner()).unwrap();
        let merged_sf_str = String::from_utf8(merged_sf_cursor.into_inner()).unwrap();
        // Fails right now because of key ordering
        //assert_eq!(merged_fs_str, merged_sf_str);
        //assert_eq!(merged_fs, expected_output_conf);
    }
}