fn perimetr_area_isravnobedr(side_1: f64, side_2: f64,side_3: f64){
    if side_1 + side_2 <= side_3 || side_1 + side_3 <= side_2 || side_2 + side_3 <= side_1{
        println!("Треугольник не существует")
    }else{
    let perimetr: f64 = side_1 + side_2 + side_3;
    println!("Периметр треугольника равен {}", perimetr);
    let half_perimetr: f64 = (side_1 + side_2 + side_3) / 2.0;
    let area: f64 = half_perimetr * (half_perimetr - side_1) * (half_perimetr - side_2) * (half_perimetr - side_3);
    println!("Площадь треугольника равна {}", area);

    if side_1 == side_2 || side_1 == side_3 || side_2 == side_3{
        println!("Треугольник равнобедренный")
    }else{
        println!("Треугольник неравнобедренный")
    }
}
}
    fn main() {
    let side_1: f64 = 3.0;
    let side_2: f64 = 8.0;
    let side_3: f64 = 8.0;
    perimetr_area_isravnobedr(side_1, side_2, side_3)
}
