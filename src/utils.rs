use euclid::Vector3D;

use super::Vertex;

fn calculate_normal(v0: [f32; 3], v1: [f32; 3], v2: [f32; 3]) -> [f32; 3] {
    let vec1: Vector3D::<f32, ()> = Vector3D::<f32, ()>::new(v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]);
    let vec2: Vector3D::<f32, ()> = Vector3D::new(v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]);

    let normal = vec1.cross(vec2);
    let normal = normal.normalize();

    [normal.x, normal.y, normal.z]
}

pub fn get_normals_for_points(vec_in: &Vec::<Vertex>) -> Vec::<[f32;3]> {
    if vec_in.len() % 3 != 0 {
        panic!("Number of vertices is not a multiple of 3, cannot calculate normals");
    };

    let mut ret = Vec::<[f32;3]>::new();
    for v in vec_in.chunks(3) {
        let v0 = v[0].position;
        let v1 = v[1].position;
        let v2 = v[2].position;

        let normal = calculate_normal(v0, v1, v2);
        ret.push(normal);
    }
    ret
}