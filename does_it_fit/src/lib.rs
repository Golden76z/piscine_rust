mod areas_volumes;
pub use areas_volumes::*;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let mut rect_area = rectangle_area(x, y);
    let shape_area: usize;
    let mut count = 0;

    match kind {
        GeometricalShapes::Rectangle => {
            shape_area = rectangle_area(a, b);
        }
        GeometricalShapes::Square => {
            shape_area = square_area(a);
        }
        GeometricalShapes::Circle => {
            shape_area = circle_area(a) as usize;
        }
        GeometricalShapes::Triangle => {
            shape_area = triangle_area(a, b) as usize;
        }
    }

    while rect_area >= shape_area {
        rect_area -= shape_area;
        count += 1;
    }

    times <= count
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let geo_vol: f64;
    let mut shape_vol: f64 = parallelepiped_volume(x, y, z) as f64;
    let mut count = 0;

    match kind {
        GeometricalVolumes::TriangularPyramid => {
            geo_vol = triangular_pyramid_volume(a as f64, b);
        }
        GeometricalVolumes::Cone => {
            geo_vol = cone_volume(a, b);
        }
        GeometricalVolumes::Sphere => {
            geo_vol = sphere_volume(a);
        }
        GeometricalVolumes::Cube => {
            geo_vol = cube_volume(a) as f64;
        }
        GeometricalVolumes::Parallelepiped => {
            geo_vol = parallelepiped_volume(a, b, c) as f64;
        }
    }

    while shape_vol >= geo_vol {
        shape_vol -= geo_vol;
        count += 1;
    }

    times <= count
}
