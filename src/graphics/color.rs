#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Color3 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Into<wgpu::Color> for Color3 {
    fn into(self) -> wgpu::Color {
        wgpu::Color {
            r: self.r as f64,
            g: self.g as f64,
            b: self.b as f64,
            a: 1.0,
        }
    }
}

impl Into<[f32; 3]> for Color3 {
    fn into(self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}

impl Into<[f32; 4]> for Color3 {
    fn into(self) -> [f32; 4] {
        [self.r, self.g, self.b, 1.]
    }
}

impl Color3 {
    pub const WHITE: Self = Self::splat(1.0);
    pub const BLACK: Self = Self::splat(0.0);
    pub const RED: Self = Self::new(1.0, 0.0, 0.0);
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0);
    pub const YELLOW: Self = Self::new(1.0, 1.0, 0.0);
    pub const CYAN: Self = Self::new(0.0, 1.0, 1.0);
    pub const MAGENTA: Self = Self::new(1.0, 0.0, 1.0);

    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub const fn splat(l: f32) -> Self {
        Self::new(l, l, l)
    }

    pub fn array_mut(&mut self) -> &mut [f32; 3] {
        unsafe { std::mem::transmute(self) }
    }
}

impl std::ops::Mul<f32> for Color3 {
    type Output = Color3;
    fn mul(self, rhs: f32) -> Self::Output {
        Color3::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl std::ops::Mul<Color3> for f32 {
    type Output = Color3;
    fn mul(self, rhs: Color3) -> Self::Output {
        Color3::new(self * rhs.r, self * rhs.g, self * rhs.b)
    }
}

impl std::ops::Add for Color3 {
    type Output = Color3;
    fn add(self, rhs: Color3) -> Self::Output {
        Color3::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl std::ops::Sub for Color3 {
    type Output = Color3;
    fn sub(self, rhs: Color3) -> Self::Output {
        Color3::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}
