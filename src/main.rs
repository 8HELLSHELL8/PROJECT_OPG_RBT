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
}
