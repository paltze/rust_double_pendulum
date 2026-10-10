# Double Pendulum Simulator

Written in Rust with Macroquad. [Quick web demo](https://paltze.github.io/rust_double_pendulum/).

## Compiling and Running

Clone the repo, or get the source in any other way. Then:

```
cargo run
cargo run --release # Release build
```

## Technical details

### Equations of motion

$$
\Delta = \theta_1 - \theta_2
$$

$$
\ddot{\theta}_1 = \frac{m_2 g \sin\theta_2 \cos\Delta - m_2 \sin\Delta \left(l_1 \dot{\theta}_1^2 \cos\Delta + l_2 \dot{\theta}_2^2\right) - (m_1 + m_2) g \sin\theta_1}{l_1 \left(m_1 + m_2 \sin^2\Delta\right)}
$$

$$
\ddot{\theta}_2 = \frac{(m_1 + m_2)\left(l_1 \dot{\theta}_1^2 \sin\Delta - g \sin\theta_2 + g \sin\theta_1 \cos\Delta\right) + m_2 l_2 \dot{\theta}_2^2 \sin\Delta \cos\Delta}{l_2 \left(m_1 + m_2 \sin^2\Delta\right)}
$$

Here $\theta_1$ and $\theta_2$ represent absolute angles of first and second arms, measured from downwards vertical

### Numerical Integration

Numerical integration to update state is carried out by RK4. Check out the Wikipedia page for [Runge-Kutta methods](https://en.wikipedia.org/wiki/Runge-Kutta_methods).

## License

This project is licensed under the MIT License. Check out LICENSE for more details.
