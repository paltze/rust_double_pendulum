# Double Pendulum Simulator

Written in Rust with Macroquad

- Individually variable lengths of rods and masses of bobs

> [Quick web demo](https://paltze.github.io/rust_double_pendulum/)

## Compiling and Running

Get the source with your preferred method, then:

```bash
cargo run

cargo run --release # Release build
```

> Must have the Rust toolchain installed

## Technical details

### Equations of motion

$$
\ddot{\theta}_1 = \frac{m_2 g \sin\theta_2 \cos\Delta - m_2 \sin\Delta \left(l_1 \dot{\theta}_1^2 \cos\Delta + l_2 \dot{\theta}_2^2\right) - (m_1 + m_2) g \sin\theta_1}{l_1 \left(m_1 + m_2 \sin^2\Delta\right)}
$$

$$
\ddot{\theta}_2 = \frac{(m_1 + m_2)\left(l_1 \dot{\theta}_1^2 \sin\Delta - g \sin\theta_2 + g \sin\theta_1 \cos\Delta\right) + m_2 l_2 \dot{\theta}_2^2 \sin\Delta \cos\Delta}{l_2 \left(m_1 + m_2 \sin^2\Delta\right)}
$$

- $g$: Gravitational acceleration
- $\theta_1, \theta_2$: Absolute angles of the arms (measured from downward vertical)
- $\Delta$: $\theta_1 - \theta_2$
- $l_1, l_2$: Lengths of the rods
- $m_1, m_2$: Masses of the bobs

### Solving the differential equations

Uses RK4, a standard fourth-order method to solve ODEs. Check out the Wikipedia page for [Runge-Kutta methods](https://en.wikipedia.org/wiki/Runge-Kutta_methods).

## License

This project is licensed under the MIT License. Check out LICENSE for more details.
