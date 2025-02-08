pub struct Composite {
    pub filename: String
}

impl Composite {
    pub fn default() -> Self {
        Composite {
            filename: "".to_string()
        }
    }

    pub fn from_string(line: &String, line_num: usize, original_filename: &String) -> Self {
        let mut first_element = line.split(",").collect::<Vec<&str>>()[0].chars();
        if first_element.next().unwrap() != '>' {
            println!("WARNING: Line number {} has been given to us but doesn't start with >", line_num);
            return Composite::default();
        }
        let filename = first_element.as_str().to_string();

        // Deconstruct the filename and give back completed
        let mut filecomponents = original_filename.split("/").collect::<Vec<&str>>(); 
        let filename = if filecomponents.len() == 1 {
                filename
            } else {
                filecomponents.pop();
                filecomponents.push(&filename);
                filecomponents.join("/")
            };
        Composite {
            filename
        }
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_composite_from_string() {
        let res = Composite::from_string(&">cube.txt,-0.5,0.5,0.5,0.7".to_string(), 0, &"".to_string());
        assert_eq!(res.filename, "cube.txt".to_string());
    }

    #[test]
    fn test_get_composite_from_string_complex_original_filename() {
        let res = Composite::from_string(&">cube.txt,-0.5,0.5,0.5,0.7".to_string(), 0, &"examples/composite.txt".to_string());
        assert_eq!(res.filename, "examples/cube.txt".to_string());
    }
}