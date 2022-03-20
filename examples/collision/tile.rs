
use barn::math::vector2::Vector2;
use barn::math::bounding_box_2d::BoundingBox2D;

#[derive(Clone)]
pub struct Tile {
    pub bb: BoundingBox2D,
    pub tile_type: TileType
}

#[derive(Clone)]
pub enum TileType {
    Ground,
    Water,
    Intangible 
}

impl Tile {

    pub fn new(tile_type: TileType, x: f32, y: f32, width: u32, height: u32, ) -> Self  {
        let bb = BoundingBox2D::new(Vector2::new(x, y), width, height);
        Self { bb: bb, tile_type: tile_type }
    }
}