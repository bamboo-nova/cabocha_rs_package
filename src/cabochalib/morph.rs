#[derive(Debug)]
pub struct Morph {
    surface: String,
    base: String,
    pos: String,
    sub_pos: String,
}

impl Morph {
    pub fn parse(result: Vec<&str>) -> Result<Vec<Vec<Self>>, std::fmt::Error> {
        let lines: Vec<_> = result.into_iter().filter(|line| if let line = line { !line.starts_with('*') } else { false }).collect();
        if lines.len() == 0 {
            return Err(std::fmt::Error);
        }
        let mut results = Vec::new();
        let mut buffer = Vec::new();
        let mut target_lines = lines.iter();
        while let Some(line) = target_lines.next() {
            if line.to_string() == "EOS" {
                if !buffer.is_empty() { results.push(buffer); }
                buffer = Vec::new();
                break;  // EOSの後に[""]の要素が出現してしまってるため、else以降が破綻するのでbreakを入れる.
            } else {
                let line = line.replace("\t", ",");
                let tmp: Vec<_> = line.split_terminator(',').collect();
                buffer.push(Self {
                    surface: tmp[0].to_string(),
                    base: tmp[7].to_string(),
                    pos: tmp[1].to_string(),
                    sub_pos: tmp[2].to_string(),
                });
            }
        }
    println!("{:?}", results);
    Ok(results)
    }
}
