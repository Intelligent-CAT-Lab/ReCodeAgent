use expr::mapper_expr;
#[test]
fn test1(){
    let expr = "y=26*2/2+log10(pi)+2.*pow(2,1*(3+7*.1)*1.1+x{-6*2+12})*3*4+cos(2.)";
    let mut e: mapper_expr::MapperExpr = mapper_expr::mapper_expr_new_from_string(expr, 1, 1, 1);
    let x = 3.0;
    let expected = 26.0 * 2.0 / 2.0 + std::f32::consts::PI.log10() + 
                   2.0 * (2.0f32.powf(1.0 * (3.0 + 7.0 * 0.1) * 1.1 + x)) * 3.0 * 4.0 +
                   2.0_f32.cos();
    let evaluated = mapper_expr::mapper_expr_evaluate(&mut e, &(mapper_expr::MapperSignalValue::F(x as f32)));
    assert!((evaluated.as_f32().unwrap() - expected).abs() < f32::EPSILON, "Test 1 failed: expected {}, got {:?}", expected, evaluated);
}
#[test]
fn test2(){
    let mut expr = "26.0*2.0/2.0 + x*30.0/(20.0*1.0)";
    println!("Parsing: {}", expr);
    for x in [3, 321] {
        let expected = 26.0 * 2.0 / 2.0 + (x as f32) * 30.0 / (20.0 * 1.0);
        let mut e: mapper_expr::MapperExpr = mapper_expr::mapper_expr_new_from_string(expr, 1, 1, 1);
        let evaluated = mapper_expr::mapper_expr_evaluate(&mut e, &(mapper_expr::MapperSignalValue::F(x as f32)));
        assert!((evaluated.as_f32().unwrap() - expected).abs() < f32::EPSILON, "Test 2 failed for x={}: expected {}, got {:?}", x, expected, evaluated);
    }
}
fn main() {
}