fn prmtr_area_diag(width: i32, height: i32){
    println!("{}", width * height);
    println!("{}", 2 * (width + height));
}
fn main() {
    println!("Hello, Igor");
    let width = 12;
    let height = 10;
    prmtr_area_diag(width, height);
}
