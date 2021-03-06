#[derive(Clone)]
pub enum CellSize {
    Auto,
    Percent(f64),
    // Px(u32),
    // Pt(u32),
    // Em(u32),
}

#[derive(Clone, Default)]
pub struct Layout {
    pub columns: Vec<CellSize>,
    pub rows: Vec<CellSize>,
    pub viewport: (u32, u32),
}

#[derive(Clone, Default)]
struct TrackSize {
    base_size: Option<f64>,
    max_growth: Option<f64>,
}

trait Layoutable {
    fn constrained_width(&self, min: f64, max: f64) -> (f64, f64, f64);
    fn constrained_height(&self, min: f64, max: f64) -> (f64, f64, f64);
}

// fn get_free_size(layout: &[CellSize], max_size: f32) -> f32 {
//     let mut w = 0.;
//     for col_w in layout {
//         w += match *col_w {
//             CellSize::Auto => 0.,
//             CellSize::Percent(p) => (p / 100.) * max_size,
//         }
//     }
//     max_size - w
// }

fn initialize_track_sizes(track_layout: &[CellSize], max_size: f64) -> Vec<TrackSize> {
    // 11.4. Initialize Track Sizes
    let tracks: Vec<TrackSize> = track_layout
        .iter()
        .map(|c| {
            let (base, growth) = match c {
                CellSize::Auto => (None, None),
                CellSize::Percent(p) => {
                    let s = (p / 100.) * max_size as f64;
                    (Some(s), Some(s))
                }
            };

            TrackSize {
                base_size: base,
                max_growth: growth,
            }
        })
        .collect();
    tracks
}

fn resolve_intrinsic_track_sizes(tracks: &mut Vec<TrackSize>, max_size: f64) {
    let free_space = tracks
        .iter()
        .fold(max_size, |free, track| match track.base_size {
            None => free,
            Some(s) => free - s,
        });
    let autos = tracks.iter().fold(0, |autos, track| match track.base_size {
        None => autos + 1,
        _ => autos,
    });
    let auto_size = free_space / f64::from(autos);

    // 11.5. Resolve Intrinsic Track Sizes
    for mut c in tracks.iter_mut() {
        if c.base_size == None {
            c.base_size = Some(auto_size);
        }
        if c.max_growth == None {
            c.max_growth = Some(auto_size);
        }
    }
}

impl Layout {
    pub fn new() -> LayoutBuilder {
        LayoutBuilder {
            columns: None,
            rows: None,
            viewport: None,
        }
    }

    pub fn get_grid(&self) -> Vec<(f64, f64, f64, f64)> {
        // COLUMNS
        // 11.4. Initialize Track Sizes
        let mut column_tracks = initialize_track_sizes(&self.columns, f64::from(self.viewport.0));

        // 11.5. Resolve Intrinsic Track Sizes
        resolve_intrinsic_track_sizes(&mut column_tracks, f64::from(self.viewport.0));

        // ROWS
        // 11.4. Initialize Track Sizes
        let mut row_tracks = initialize_track_sizes(&self.rows, f64::from(self.viewport.1));

        // 11.5. Resolve Intrinsic Track Sizes
        resolve_intrinsic_track_sizes(&mut row_tracks, f64::from(self.viewport.1));

        //
        let mut res: Vec<(f64, f64, f64, f64)> = vec![];
        let mut row_pos = 0.;
        let mut col_pos = 0.;
        for r in &row_tracks {
            for c in &column_tracks {
                res.push((
                    col_pos,
                    row_pos,
                    col_pos + c.base_size.unwrap(),
                    row_pos + r.base_size.unwrap(),
                ));
                col_pos += c.base_size.unwrap();
            }
            col_pos = 0.;
            row_pos += r.base_size.unwrap();
        }
        res
    }
}

#[derive(Clone, Default)]
pub struct LayoutBuilder {
    columns: Option<Vec<CellSize>>,
    rows: Option<Vec<CellSize>>,
    viewport: Option<(u32, u32)>,
}

impl LayoutBuilder {
    // pub fn new() -> Self {
    //     GridLayoutBuilder {
    //         columns: None,
    //         rows: None,
    //         viewport: None,
    //     }
    // }

    pub fn with_columns(mut self, cols: &[CellSize]) -> Self {
        self.columns = Some(cols.to_vec());
        self
    }

    pub fn with_rows(mut self, rows: &[CellSize]) -> Self {
        self.rows = Some(rows.to_vec());
        self
    }

    pub fn with_viewport(mut self, width: u32, height: u32) -> Self {
        self.viewport = Some((width, height));
        self
    }

    pub fn build(&self) -> Result<Layout, &'static str> {
        if self.viewport == None {
            return Err("ViewPort size not defined.");
        }
        let viewport = self.viewport.unwrap();
        Ok(Layout {
            columns: match self.columns.clone() {
                Some(cols) => cols,
                None => vec![CellSize::Auto],
            },
            rows: match self.rows.clone() {
                Some(rows) => rows,
                None => vec![CellSize::Auto],
            },
            viewport,
        })
    }
}
