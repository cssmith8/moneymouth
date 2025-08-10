use crate::types::simpledate::Simpledate;

pub fn date_from_string(input: String) -> Result<Simpledate, String> {
    let parts: Vec<&str> = input.split(&['-', '/'][..]).collect();

    if parts.len() != 3 {
        return Err("Invalid date format. Expected 3 parts separated by - or /".into());
    }

    let part1: i32 = parts[0]
        .parse()
        .map_err(|_| "Invalid number in date".to_string())?;
    let part2: u32 = parts[1]
        .parse()
        .map_err(|_| "Invalid number in date".to_string())?;
    let part3: u32 = parts[2]
        .parse()
        .map_err(|_| "Invalid number in date".to_string())?;

    // If first part is 1-12, assume m/d/y format, otherwise y/m/d
    if part1 >= 1 && part1 <= 12 && part3 <= 99 {
        // m/d/y format - handle 2-digit years
        let year = if part3 < 100 {
            2000 + part3 as i32
        } else {
            part3 as i32
        };
        Ok(Simpledate {
            year,
            month: part1 as u32,
            day: part2,
        })
    } else {
        // y/m/d format - handle 2-digit years
        let year = if part1 < 100 { 2000 + part1 } else { part1 };
        Ok(Simpledate {
            year,
            month: part2,
            day: part3,
        })
    }
}
