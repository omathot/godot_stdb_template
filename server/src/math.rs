use spacetimedb::SpacetimeType;

#[derive(SpacetimeType, Debug, Clone, Copy)]
pub struct DbVec2 {
    pub x: f32,
    pub y: f32,
}

impl std::ops::Add<&DbVec2> for DbVec2 {
    type Output = DbVec2;

    fn add(self, rhs: &DbVec2) -> Self::Output {
        DbVec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Add<DbVec2> for DbVec2 {
    type Output = DbVec2;

    fn add(self, rhs: DbVec2) -> Self::Output {
        DbVec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign<DbVec2> for DbVec2 {
    fn add_assign(&mut self, rhs: DbVec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::iter::Sum<DbVec2> for DbVec2 {
    fn sum<I: Iterator<Item = DbVec2>>(iter: I) -> Self {
        let mut r = DbVec2::new(0.0, 0.0);
        for val in iter {
            r += val;
        }
        r
    }
}

impl std::ops::Sub<&DbVec2> for DbVec2 {
    type Output = DbVec2;

    fn sub(self, rhs: &DbVec2) -> Self::Output {
        DbVec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Sub<DbVec2> for DbVec2 {
    type Output = DbVec2;

    fn sub(self, rhs: DbVec2) -> Self::Output {
        DbVec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::SubAssign<DbVec2> for DbVec2 {
    fn sub_assign(&mut self, rhs: DbVec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Mul<f32> for DbVec2 {
    type Output = DbVec2;

    fn mul(self, rhs: f32) -> Self::Output {
        DbVec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Div<f32> for DbVec2 {
    type Output = DbVec2;

    fn div(self, rhs: f32) -> Self::Output {
        DbVec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DbVec2 {
    pub fn new(x: f32, y: f32) -> DbVec2 {
        DbVec2 { x, y }
    }

    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(self) -> DbVec2 {
        self / self.magnitude()
    }
}
