#[derive(PartialEq, Clone, Copy)]
pub struct V(pub f64, pub f64);

impl V {
    pub fn new(x: f64, y: f64) -> V {
        V(x, y)
    }
    pub fn norm(&self) -> f64 {
        V::dot(*self, *self).sqrt()
    }
    pub fn dot(a: V, b: V) -> f64 {
        a.0 * b.0 + a.1 * b.1
    }
}

impl std::ops::Neg for V {
    type Output = V;
    fn neg(self) -> V {
        V(-self.0, -self.1)
    }
}

impl std::ops::Add for V {
    type Output = V;
    fn add(self, b: V) -> V {
        V(self.0 + b.0, self.1 + b.1)
    }
}

impl std::ops::AddAssign for V {
    fn add_assign(&mut self, b: V) {
        self.0 += b.0;
        self.1 += b.1;
    }
}

impl std::ops::Sub for V {
    type Output = V;
    fn sub(self, b: V) -> V {
        V(self.0 - b.0, self.1 - b.1)
    }
}

impl std::ops::SubAssign for V {
    fn sub_assign(&mut self, b: V) {
        self.0 -= b.0;
        self.1 -= b.1;
    }
}

impl std::ops::Mul<f64> for V {
    type Output = V;
    fn mul(self, b: f64) -> V {
        V(self.0 * b, self.1 * b)
    }
}

impl std::ops::Mul<V> for f64 {
    type Output = V;
    fn mul(self, b: V) -> V {
        V(self * b.0, self * b.1)
    }
}

impl std::ops::Div<f64> for V {
    type Output = V;
    fn div(self, b: f64) -> V {
        V(self.0 / b, self.1 / b)
    }
}

impl std::ops::DivAssign<f64> for V {
    fn div_assign(&mut self, b: f64) {
        self.0 /= b;
        self.1 /= b;
    }
}

impl std::ops::MulAssign<f64> for V {
    fn mul_assign(&mut self, b: f64) {
        self.0 *= b;
        self.1 *= b;
    }
}

impl std::fmt::Display for V {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.3}, {:.3})", self.0, self.1)
    }
}

impl std::fmt::Debug for V {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
