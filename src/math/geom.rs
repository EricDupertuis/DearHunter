pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Rect {
    pub center: Point,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn contains(&self, p: &Point) -> bool {
        let left = self.center.x - self.width;
        let right = self.center.x + self.width;
        let bottom = self.center.y - self.height;
        let top = self.center.y + self.height;

        p.x >= left && p.x <= right && p.y >= bottom && p.y <= top
    }
}
