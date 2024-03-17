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

}
