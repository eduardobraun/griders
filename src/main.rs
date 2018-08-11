#![warn(clippy)]

// extern crate rand;
extern crate svg;

mod grid;

// use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

use grid::*;

fn main() {
    use CellSize::*;
    let grid_layout = GridLayout::new()
        .with_columns(&[Percent(20.), Auto, Auto, Percent(20.)])
        .with_rows(&[Percent(15.), Auto])
        .with_viewport(800, 600)
        .build()
        .expect("Could not create layout");

    let mut document = Document::new().set(
        "viewBox",
        (0, 0, grid_layout.viewport.0, grid_layout.viewport.1),
    );

    let colors = [
        "#ff0000", "#00ff00", "#0000ff", "#ffff00", "#00ffff", "#ff00ff", "#7f18ee", "#ee187f",
    ];

    let grid = grid_layout.get_grid();

    for (i, &cell) in grid.iter().enumerate() {
        println!{"Cell {:?}", cell};
        let (x1, y1, x2, y2) = cell;
        let data = Data::new()
            .move_to((x1, y1))
            .line_to((x1, y2))
            .line_to((x2, y2))
            .line_to((x2, y1))
            .close();

        let color = *colors.get(i).unwrap();
        let path = Path::new()
            .set("fill", color)
            .set("stroke", "none")
            .set("stroke-width", 3)
            .set("d", data);
        document = document.add(path);
    }

    svg::save("image.svg", &document).unwrap();
}
