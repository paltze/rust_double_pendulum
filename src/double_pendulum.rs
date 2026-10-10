use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct State {
    pub theta1: f64,
    pub omega1: f64,
    pub theta2: f64,
    pub omega2: f64,
}

impl Add for State {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            theta1: self.theta1 + rhs.theta1,
            omega1: self.omega1 + rhs.omega1,
            theta2: self.theta2 + rhs.theta2,
            omega2: self.omega2 + rhs.omega2,
        }
    }
}

impl Mul<f64> for State {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f64) -> Self {
        Self {
            theta1: self.theta1 * scalar,
            omega1: self.omega1 * scalar,
            theta2: self.theta2 * scalar,
            omega2: self.omega2 * scalar,
        }
    }
}

fn get_alpha(l1: f64, m1: f64, l2: f64, m2: f64, s: State) -> (f64, f64) {
    let g: f64 = 9.81;
    let delta = s.theta1 - s.theta2;

    // Precomputations
    let sin_delta = delta.sin();
    let cos_delta = delta.cos();

    let sin_theta1 = s.theta1.sin();
    let sin_theta2 = s.theta2.sin();

    let a1 = l1 * s.omega1 * s.omega1;
    let a2 = l2 * s.omega2 * s.omega2;

    let denominator = m1 + m2 * sin_delta * sin_delta;

    // alpha1
    let mut alpha1 = a1 * cos_delta + a2;
    alpha1 *= -(m2 * sin_delta);
    alpha1 += m2 * g * sin_theta2 * cos_delta;
    alpha1 -= (m1 + m2) * g * sin_theta1;
    alpha1 /= l1 * denominator;

    // alpha2
    let mut alpha2 = a1 * sin_delta;
    alpha2 -= g * sin_theta2;
    alpha2 += g * sin_theta1 * cos_delta;
    alpha2 *= m1 + m2;
    alpha2 += m2 * a2 * sin_delta * cos_delta;
    alpha2 /= l2 * denominator;

    (alpha1, alpha2)
}

#[inline]
fn f(l1: f64, m1: f64, l2: f64, m2: f64, s: State) -> State {
    let (alpha1, alpha2) = get_alpha(l1, m1, l2, m2, s);

    State {
        theta1: s.omega1,
        omega1: alpha1,
        theta2: s.omega2,
        omega2: alpha2,
    }
}

pub fn rk4_step(l1: f64, m1: f64, l2: f64, m2: f64, state: &mut State, dt: f64) {
    let k1 = f(l1, m1, l2, m2, *state);
    let k2 = f(l1, m1, l2, m2, *state + k1 * (0.5 * dt));
    let k3 = f(l1, m1, l2, m2, *state + k2 * (0.5 * dt));
    let k4 = f(l1, m1, l2, m2, *state + k3 * dt);

    *state = *state + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (dt / 6.0);
}

/*pub fn step(l1: f64, m1: f64, l2: f64, m2: f64, state: &mut State, dt: f64) {
    let (alpha1, alpha2) = get_alpha(l1, m1, l2, m2, *state);

    state.omega1 += alpha1 * dt;
    state.omega2 += alpha2 * dt;

    state.theta1 += state.omega1 * dt;
    state.theta2 += state.omega2 * dt;
}*/
