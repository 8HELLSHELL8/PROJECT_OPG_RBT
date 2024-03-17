fn trapeze(little_base:i32, big_base:i32, left_side:i32, right_side:i32, height:i32){
    //perimetr
    let perimetr: i32 = little_base + big_base + left_side + right_side;

    //middle line
    let middle_line: f64 = ((little_base as f64) + (big_base as f64))/2.0;

    //area
    let area:f64 = middle_line * (height as f64); 

    println!("Данные трапеции:");
    println!("Периметр = {} ", perimetr);
    println!("Площадь = {} ", area);
    println!("Длина средней линии = {} ", middle_line);

}


fn main() {
    trapeze(14,26, 10, 10, 8);
}
