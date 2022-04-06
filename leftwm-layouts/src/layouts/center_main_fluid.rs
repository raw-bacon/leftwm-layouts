use crate::{
    geometry::Rect,
    Layout, LayoutModifiers
};

/// ## Demonstration
/// 1 window
/// ```text
/// +-----+-----------+-----+
/// |     |           |     |
/// |     |           |     |
/// |     |     1     |     |
/// |     |           |     |
/// |     |           |     |
/// +-----+-----------+-----+
/// ```
/// 2 windows
/// ```text
/// +-----+-----------+-----+
/// |     |           |     |
/// |     |           |     |
/// |  2  |     1     |     |
/// |     |           |     |
/// |     |           |     |
/// +-----+-----------+-----+
/// ```
/// 3 windows
/// ```text
/// +-----+-----------+-----+
/// |     |           |     |
/// |     |           |     |
/// |  2  |     1     |  3  |
/// |     |           |     |
/// |     |           |     |
/// +-----+-----------+-----+
/// ```
/// 4 windows
/// ```text
/// +-----+-----------+-----+
/// |     |           |  3  |
/// |     |           |     |
/// |  2  |     1     +-----+
/// |     |           |  4  |
/// |     |           |     |
/// +-----+-----------+-----+
/// ```
/// 5 windows
/// ```text
/// +-----+-----------+-----+
/// |     |           |  3  |
/// |     |           +-----+
/// |  2  |     1     |  4  |
/// |     |           +-----+
/// |     |           |  5  |
/// +-----+-----------+-----+
/// ```
#[derive(Debug)]
pub struct CenterMainFluid;

impl Layout for CenterMainFluid {
    fn apply(
        &self,
        window_count: usize,
        _container: Rect,
        _modifiers: &LayoutModifiers,
    ) -> Vec<Rect> {
        let tiles: &mut Vec<Rect> = &mut Vec::new();
        if window_count == 0 {
            return tiles.to_vec();
        }
        todo!()
    }
}

mod tests {}
