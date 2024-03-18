fn trapeze(little_base:i32, big_base:i32, left_side:i32, right_side:i32, height:i32){
   
    //ПЕРИМЕТР
    let perimetr: i32 = little_base + big_base + left_side + right_side; //Рассчет суммы длин всех сторон

    //СРЕДНЯЯ ЛИНИЯ
    let middle_line: f64 = ((little_base as f64) + (big_base as f64))/2.0; //Рассчет средней линии

    //ПЛОЩАДЬ
    let area:f64 = middle_line * (height as f64); //Рассчет площади через среднюю линию и высоту

    //Вывод 
    println!("Данные трапеции:");
    println!("Периметр = {} ", perimetr);
    println!("Площадь = {} ", area);
    println!("Длина средней линии = {} ", middle_line); //добавляем комментарий к коду вывода
}

fn perimetr_area_isravnobedr(side_1: f64, side_2: f64,side_3: f64){ //Функция выводит на экран значения периметра, площади и проверяет на равнобедренность
    if side_1 + side_2 <= side_3 || side_1 + side_3 <= side_2 || side_2 + side_3 <= side_1{ // Проверка на существование треугольника
        println!("Треугольник не существует")
    }else{
    let perimetr: f64 = side_1 + side_2 + side_3; // Вычисление периметра исходного треугольника
    println!("Периметр треугольника равен {}", perimetr);
    let half_perimetr: f64 = (side_1 + side_2 + side_3) / 2.0; // Вычисление полупериметра исходного треугольника
    let area: f64 = half_perimetr * (half_perimetr - side_1) * (half_perimetr - side_2) * (half_perimetr - side_3); // Вычисление площади треугольника по формуле Герона
    println!("Площадь треугольника равна {}", area);

    if side_1 == side_2 || side_1 == side_3 || side_2 == side_3{ // Проверка на равнобедренность треугольника
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
    println!("Длина диагонали прямоугольника равна {}", (diag_is_sqrd as f64).sqrt());//Вывод диагонали прямоугольника
}


fn main() {
    prmtr_area_diag(19, 10);//Вызов функции prmtr_area_diag
    println!("");
    trapeze(14,26, 10, 10, 8);//Вызов функции trapeze
    println!("");
    perimetr_area_isravnobedr(4.0, 8.0, 8.0);//Вызов функции perimetr_area_isravnobedr
    println!("IT WORKS!");
}

