use std::{
    error::Error,
    ffi::OsString,
    fs::{self, File},
    io::Write,
};
use std::path::PathBuf;


fn main() -> Result<(), Box<dyn Error>> {
    let years_path = PathBuf::new().join("src").join("years");
    println!("cargo:rerun-if-changed=src/years");
    let mut years_mod_file = File::create(years_path.join("mod.rs"))?;
    for year in fs::read_dir(&years_path)? {
        let Ok(year) = year else { continue; };
        let year_name = year.file_name();
        let Some(year_name) = year_name.to_str() else { continue; };
        if year_name != OsString::from("mod.rs") {
            writeln!(years_mod_file, "pub mod {year_name};")?;
            let mut year_mod_file = File::create(year.path().join("mod.rs"))?;

            for day in fs::read_dir(year.path())? {
                let Ok(day) = day else { continue; };
                if day.file_name() != OsString::from("mod.rs") {
                    let day_name = day.path();
                    let Some(day_name) = day_name.file_stem() else { continue; };
                    let Some(day_name) = day_name.to_str() else { continue; };
                    writeln!(year_mod_file, "pub mod {day_name};")?;
                    println!("{:?}", day_name);
                }
            }

        }
    }
    for year in fs::read_dir(years_path)? {
        let Ok(year) = year else { continue; };
        let year_name = year.file_name();
        let Some(year_name) = year_name.to_str() else { continue; };
        if year_name != OsString::from("mod.rs") {
            println!("{:?}", year.path().join("mod.rs"));
            let mut year_mod_file = File::options().append(true).open(year.path().join("mod.rs"))?;
            writeln!(year_mod_file, "#[cfg(test)]")?;
            writeln!(year_mod_file, "mod tests {{")?;
            writeln!(year_mod_file, "   mod {} {{", year_name)?;
            for day in fs::read_dir(year.path())? {

                let Ok(day) = day else { continue; };
                if day.file_name() != OsString::from("mod.rs") {
                    let day_name = day.path();
                    let Some(day_name) = day_name.file_stem() else { continue; };
                    let Some(day_name) = day_name.to_str() else { continue; };
                    writeln!(year_mod_file, "        crate::test_test_existence!({year_name}, {day_name});")?;
                }
            }
            writeln!(year_mod_file, "    }}")?;
            writeln!(year_mod_file, "}}")?;
        }
    }
    Ok(())
}