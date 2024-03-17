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




fn prmtr_area_diag(width: i32, height: i32){//Функция выводит на экран значения периметра, площади, длины диагонали прямоугольника
    println!("Площадь прямоугольника равна {}", width * height);//Вывод площади
    println!("Периметр прямоугольника равен {}", 2 * (width + height));//Вывод периметра
    let diag_is_sqrd: i32 = width.pow(2) + height.pow(2);//Переменная diag_is_sqrd - длина диагонали в квадрате
    println!("Длина диагонали прямоугольника равна {}", (diag_is_sqrd as f64).sqrt());//Вывод диагонали
}



fn main() {
    let width: i32 = 19;//Ширина прямоугольника
    let height: i32 = 10;//Высота прямоугольника
    prmtr_area_diag(width, height);//Вызов функции
    println!("");
    trapeze(14,26, 10, 10, 8);
    println!("IT WORKS!");
    println!("");
    let side_1: f64 = 3.0;
    let side_2: f64 = 8.0;
    let side_3: f64 = 5.0;
    perimetr_area_isravnobedr(side_1, side_2, side_3);
}
   
