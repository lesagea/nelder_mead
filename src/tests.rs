use crate::minimizer::Minimizer;

#[test]
fn minimize_square_1d() {
    let minimizer = Minimizer::<[f64; 1]>::default();
    let expected = 0.0;
    let result = minimizer.minimize(&[1.0], |x| x[0].powi(2)).unwrap();

    println!("f_min = {:?}", result.f_min);
    println!("x_min = {:?}", result.x_min);
    println!(" iter = {:?}", result.iter);

    assert!((result.f_min - expected).abs() < 1e-9);
}

#[test]
fn minimize_square_2d() {
    let minimizer = Minimizer::<[f64; 2]>::default();
    let expected = 0.0;
    let result = minimizer
        .minimize(&[1.0, 1.0], |x| x[0].powi(2) * x[1].powi(2))
        .unwrap();

    println!("f_min = {:?}", result.f_min);
    println!("x_min = {:?}", result.x_min);
    println!(" iter = {:?}", result.iter);

    assert!((result.f_min - expected).abs() < 1e-9);
}

#[test]
fn minimize_sin() {
    let minimizer = Minimizer::<[f64; 1]>::default();
    let expected = -1.0;
    let result = minimizer.minimize(&[0.0], |x| x[0].sin()).unwrap();

    println!("f_min = {:?}", result.f_min);
    println!("x_min = {:?}", result.x_min);
    println!(" iter = {:?}", result.iter);

    assert!((result.f_min - expected).abs() < 1e-9);
}

#[test]
fn minimize_cosh() {
    let minimizer = Minimizer::<[f64; 1]>::default();
    let expected = 1.0;
    let result = minimizer.minimize(&[1.0], |x| x[0].cosh()).unwrap();

    println!("f_min = {:?}", result.f_min);
    println!("x_min = {:?}", result.x_min);
    println!(" iter = {:?}", result.iter);

    assert!((result.f_min - expected).abs() < 1e-9);
}

#[test]
fn minimize_abs() {
    let minimizer = Minimizer::<[f64; 1]>::default();
    let expected = 0.0;
    let result = minimizer.minimize(&[1.0], |x| x[0].abs()).unwrap();

    println!("f_min = {:?}", result.f_min);
    println!("x_min = {:?}", result.x_min);
    println!(" iter = {:?}", result.iter);

    assert!((result.f_min - expected).abs() < 1e-9);
}
