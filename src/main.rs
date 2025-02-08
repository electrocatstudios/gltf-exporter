use gltf_json as json;

use std::{fs, mem, vec, env};

use json::validation::Checked::Valid;
use std::borrow::Cow;
use std::io::{Write}; //BufReader
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use json::validation::USize64;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Output {
    /// Output standard glTF.
    Standard,

    /// Output binary glTF.
    Binary,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
    normal: [f32; 2]
}

/// Calculate bounding coordinates of a list of vertices, used for the clipping distance of the model
fn bounding_coords(points: &[Vertex]) -> ([f32; 3], [f32; 3]) {
    let mut min = [f32::MAX, f32::MAX, f32::MAX];
    let mut max = [f32::MIN, f32::MIN, f32::MIN];

    for point in points {
        let p: [f32; 3] = point.position;
        for i in 0..3 {
            min[i] = f32::min(min[i], p[i]);
            max[i] = f32::max(max[i], p[i]);
        }
    }
    (min, max)
}

fn align_to_multiple_of_four(n: &mut u32) {
    *n = (*n + 3) & !3;
}

fn to_padded_byte_vector<T>(vec: Vec<T>) -> Vec<u8> {
    let byte_length = vec.len() * mem::size_of::<T>();
    let byte_capacity = vec.capacity() * mem::size_of::<T>();
    let alloc = vec.into_boxed_slice();
    let ptr = Box::<[T]>::into_raw(alloc) as *mut u8;
    let mut new_vec = unsafe { Vec::from_raw_parts(ptr, byte_length, byte_capacity) };
    while new_vec.len() % 4 != 0 {
        new_vec.push(0); // pad to multiple of four bytes
    }
    new_vec
}

fn export(output: Output, triangle_vertices: Vec::<Vertex>, filename: String) {
    
    let (min, max) = bounding_coords(&triangle_vertices);

    let mut root = gltf_json::Root::default();

    let buffer_length = triangle_vertices.len() * mem::size_of::<Vertex>();
    let buffer = root.push(json::Buffer {
        byte_length: USize64::from(buffer_length),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        uri: if output == Output::Standard {
            let bin_fn = format!("{}.bin", &filename.to_string());
            println!("{}",bin_fn);
            Some(bin_fn)
        } else {
            None
        },
    });

    let buffer_view = root.push(json::buffer::View {
        buffer,
        byte_length: USize64::from(buffer_length),
        byte_offset: None,
        byte_stride: Some(json::buffer::Stride(mem::size_of::<Vertex>())),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(json::buffer::Target::ArrayBuffer)),
    });
    let positions = root.push(json::Accessor {
        buffer_view: Some(buffer_view),
        byte_offset: Some(USize64(0)),
        count:USize64::from(triangle_vertices.len()),
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec3),
        min: Some(json::Value::from(Vec::from(min))),
        max: Some(json::Value::from(Vec::from(max))),
        name: None,
        normalized: false,
        sparse: None,
    });
    let colors = root.push(json::Accessor {
        buffer_view: Some(buffer_view),
        byte_offset: Some(USize64::from(3 * mem::size_of::<f32>())),
        count: USize64::from(triangle_vertices.len()),
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec3),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });

    let normals = root.push(json::Accessor {
        buffer_view: Some(buffer_view),
        byte_offset: Some(USize64::from(6 * mem::size_of::<f32>())),
        count: USize64::from(triangle_vertices.len()),
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec2),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });

    let primitive = json::mesh::Primitive {
        attributes: {
            let mut map = std::collections::BTreeMap::new();
            map.insert(Valid(json::mesh::Semantic::Positions), positions);
            map.insert(Valid(json::mesh::Semantic::Colors(0)), colors);
            map.insert(Valid(json::mesh::Semantic::TexCoords(0)), normals);
            map
        },
        extensions: Default::default(),
        extras: Default::default(),
        indices: None,
        material: None,
        mode: Valid(json::mesh::Mode::Triangles),
        targets: None,
    };

    let mesh = root.push(json::Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        name: Some(filename.to_string() + "_mesh"),
        primitives: vec![primitive],
        weights: None,
    });

    let node = root.push(json::Node {
        mesh: Some(mesh),
        ..Default::default()
    });
    
    root.push(json::Scene {
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        nodes: vec![node],
    });

    match output {
        Output::Standard => {
            let _ = fs::create_dir("./output/".to_owned() + &filename.to_string());

            let writer = fs::File::create("./output/".to_owned() + &filename.to_string() + "/" + &filename.to_string() + ".gltf").expect("I/O error");
            json::serialize::to_writer_pretty(writer, &root).expect("Serialization error");

            let bin = to_padded_byte_vector(triangle_vertices);
            let mut writer = fs::File::create("./output/".to_owned() + &filename.to_string() + "/" + &filename.to_string() + ".bin").expect("I/O error");
            writer.write_all(&bin).expect("I/O error");
        }
        Output::Binary => {
            let json_string = json::serialize::to_string(&root).expect("Serialization error");
            let mut json_offset = json_string.len() as u32;
            align_to_multiple_of_four(&mut json_offset);
            let glb = gltf::binary::Glb {
                header: gltf::binary::Header {
                    magic: *b"glTF",
                    version: 2,
                    length: (json_offset + buffer_length as u32)
                    .try_into()
                    .expect("file size exceeds binary glTF limit"),
                },
                bin: Some(Cow::Owned(to_padded_byte_vector(triangle_vertices))),
                json: Cow::Owned(json_string.into_bytes()),
            };
            let writer = std::fs::File::create("./output/".to_owned() + &filename.to_string() + "/" + &filename.to_string() + ".glb").expect("I/O error");
            glb.to_writer(writer).expect("glTF binary output error");
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut triangle_vertices: Vec::<Vertex> = Vec::new();
    
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    if args.len() < 2 {
        println!("Please pass in a filename to input");
        return;
    }

    let filepath = &args[1];
    let binding = filepath.to_string();
    let path = Path::new(&binding);
    if !path.exists() {
        println!("File does not exist");
        return;
    }
    let filecomponents = filepath.split("/").collect::<Vec<&str>>().last().expect("Split incorrectly").to_string(); 
    let filename = filecomponents.split(".").collect::<Vec<&str>>().first().expect("Something went wrong getting filename").to_string();

    // Read in file name for passed arguments
    if let Ok(lines) = read_lines(filename.to_string() + ".txt") {
        for (idx, line_buf) in lines.enumerate() {
            if let Ok(line) = line_buf {
                let split_line = line.split(",");
                let vec = split_line.collect::<Vec<&str>>();
                if vec.len() == 8 {
                    // Points x,y,z
                    let pt1: f32 = vec[0].trim().parse::<f32>().unwrap();
                    let pt2: f32 = vec[1].trim().parse::<f32>().unwrap();
                    let pt3: f32 = vec[2].trim().parse::<f32>().unwrap();
                    // Colors r,g,b
                    let red: f32 = vec[3].trim().parse::<f32>().unwrap();
                    let green: f32 = vec[4].trim().parse::<f32>().unwrap();
                    let blue: f32 = vec[5].trim().parse::<f32>().unwrap();

                    // Texture Coordinates
                    let uv_x: f32 = vec[6].trim().parse::<f32>().unwrap();
                    let uv_y: f32 = vec[7].trim().parse::<f32>().unwrap();
                    
                    let vtx = Vertex{
                        position: [pt1, pt2, pt3],
                        color: [red, green, blue],
                        normal: [uv_x, uv_y],
                    };
                    triangle_vertices.push(vtx);
                }else{
                    println!("Skipping line {} as it is {} items long", idx, vec.len());
                }
                
            }
        }
    }
    export(Output::Standard, triangle_vertices.to_owned(), filename.to_string());
    export(Output::Binary, triangle_vertices.to_owned(), filename.to_string());
}
