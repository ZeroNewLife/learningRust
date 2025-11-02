fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
fn subtract_numbers(a: i32, b: i32) -> i32 {
    return a - b;
}

fn main() {
    let zen = add_numbers(11, 100);
    println!(" a + b = {zen}");

    let sub = subtract_numbers(11, 100);
    println!(" a - b = {sub}");

    //Эти дядки будут либо 32 либо 64 бита в зависимости от архитектуры
    let iz :isize =123;
    let iu :usize =145;
    println!(" iz = {iz}, iu = {iu} ");

    //Для залупы вроде эмодзи есть тип char 
    let ch1: char = 'A';
    let ch2: char = 'Ж';
    let ch3: char = '😀';
    println!(" ch1 = {ch1}, ch2 = {ch2}, ch3 = {ch3} ");

    //Scan Min AND Max values for integer types
    let i_min = std::i32::MIN;
    let i_max = std::i32::MAX;
    print!(" i32 min = {i_min}, i32 max = {i_max}");

}
