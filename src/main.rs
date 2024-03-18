fn prmtr_area_diag(width: i32, height: i32){//Функция выводит на экран значения периметра, площади, длины диагонали прямоугольника
    println!("Площадь прямоугольника равна {}", width * height);//Вывод площади
    println!("Периметр прямоугольника равен {}", 2 * (width + height));//Вывод периметра
    let diag_is_sqrd: i32 = width.pow(2) + height.pow(2);//Переменная diag_is_sqrd - длина диагонали в квадрате
    println!("Длина диагонали прямоугольника равна {}", (diag_is_sqrd as f64).sqrt());//Вывод диагонали прямоугольника
}
fn main() {
    prmtr_area_diag(19, 10);//Вызов функции prmtr_area_diag
}
