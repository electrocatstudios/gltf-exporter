pub struct OffsetPoint3d<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> OffsetPoint3d<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        OffsetPoint3d { x: x, y: y, z: z }
    }
}

pub struct Offset {
    pub loc: OffsetPoint3d<f32>,
    pub rot: OffsetPoint3d<f32>
}

impl Offset {
    pub fn default() -> Self {
        Offset{
            loc: OffsetPoint3d::new(0.0,0.0,0.0),
            rot: OffsetPoint3d::new(0.0,0.0,0.0),
        }
    }

    pub fn from_string(line: &String, line_num: usize) -> Self {
        let components = line.split(",").collect::<Vec<&str>>();
        if components.len() != 5 && components.len() != 7 {
            println!("WARNING: line {}, malformed offsets, using default", line_num);
            return Offset::default();
        }

        let x = components[1].trim().parse::<f32>().unwrap();
        let y = components[2].trim().parse::<f32>().unwrap();
        let z = components[3].trim().parse::<f32>().unwrap();
        let rot = if components.len() == 5 {
            OffsetPoint3d::new(
                0.0,
                components[4].trim().parse::<f32>().unwrap(),
                0.0
            )
        } else {
            OffsetPoint3d::new(
                components[4].trim().parse::<f32>().unwrap(),
                components[5].trim().parse::<f32>().unwrap(),
                components[6].trim().parse::<f32>().unwrap()
            )
        };
        
        Offset{
            loc: OffsetPoint3d::new(x,y,z),
            rot: rot
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_offset_from_string() {
        let res = Offset::from_string(&">./cube.txt,-0.5,0.5,0.5,0.7".to_string(), 0);
        assert_eq!(res.loc.x, -0.5);
        assert_eq!(res.loc.y, 0.5);
        assert_eq!(res.loc.z, 0.5);
        assert_eq!(res.rot.y,0.7);
    }

    #[test]
    fn test_get_offset_from_string_malformed() {
        let res = Offset::from_string(&">./cube.txt,-0.5,0.5".to_string(), 123);
        assert_eq!(res.loc.x, 0.0);
        assert_eq!(res.loc.y, 0.0);
        assert_eq!(res.loc.z, 0.0);
        assert_eq!(res.rot.x,0.0);
        assert_eq!(res.rot.y,0.0);
        assert_eq!(res.rot.z,0.0);
    }

    #[test]
    fn test_get_offset_from_string_3_axis_rot() {
        let res = Offset::from_string(&">./cube.txt,-0.5,0.1,0.4,0.7,0.8,0.9".to_string(), 0);
        assert_eq!(res.loc.x, -0.5);
        assert_eq!(res.loc.y, 0.1);
        assert_eq!(res.loc.z, 0.4);
        
        assert_eq!(res.rot.x,0.7);
        assert_eq!(res.rot.y,0.8);
        assert_eq!(res.rot.z,0.9);
    }
}