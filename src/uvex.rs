use crate::detector::Detector;
use astroimsim_geometry::*;
use astroimsim_geometry::coordinate_system::{CoordinateSystem, Coordinates};
use astroimsim_geometry::grid2d::GRID2D;
use astroimsim_geometry::points::Point;

pub fn empty_fuv() -> GRID2D {

    let coord = CoordinateSystem{
        x_axis: (1.0,0.0),
        y_axis: (0.0,1.0),
        center: (0.0, 0.0),
        color: "red",
        label: "fuv",
    };
    let mut grid = GRID2D::new_empty(
        (18,18), //x_num
        (0.2,0.2), //x_step_size
        (-0.56, -0.06), //y_num
        0.1, //y_step_size
        Coordinates::RELATIVE(coord)
    );
    grid.label = "fuv".to_string();
    grid
}



pub fn new_uvex(label: String, center:Point,num_pixels:usize,coordinates: Coordinates) -> Detector {

    let grid = GRID2D::new_empty((num_pixels, num_pixels), (1.0, 1.0), center.convert(&coordinates).values(), 0.001, coordinates);
    let mut data = Vec::with_capacity(num_pixels*num_pixels);
    for _row in 0..num_pixels{
        let mut row_vec = Vec::with_capacity(num_pixels);
        for _column in 0..num_pixels{
            row_vec.push(0.0)
        }
        data.push(row_vec);
    }
    Detector {label, grid, data}
}