fn test_loss() {
    let y = matrix![0.2, 0.1, 0.7; 1.0, 0.0, 0.0; 0.4, 0.5, 0.1f32];
    let t = matrix![0.0, 0.0, 1.0; 1.0, 0.0, 0.0; 1.0, 0.0, 0.0];

    println!("MSE: {}", loss::mean_squared_error(&y, &t));
    println!("CEE: {}", loss::cross_entropy_error(&y, &t));
}
