fn prmtr_area_diag(width: i32, height: i32){
    println!("{}", width * height);
    println!("{}", 2 * (width + height));
    let diag_is_sqrd: i32 = width.pow(2) + height.pow(2);
    println!("{}", (diag_is_sqrd as f64).sqrt());
}
fn main() {
    let width: i32 = 19;
    let height: i32 = 10;
    prmtr_area_diag(width, height);
}
