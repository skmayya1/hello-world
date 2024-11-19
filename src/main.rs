fn main() {
    hello();
    data_types();
}
fn hello(){
    println!("Hello, world!");
}

fn data_types(){
    //INT
    let num:u8 =4;
    println!("num: {} ", num); 
    let mut num2:i8 = 4;
    println!("num2: {} ", num2);
    num2 = 5;
    println!("num2: {} ", num2);

    //STRING (String is a growable, heap-allocated data structure)
    //&str is a Fixed length string --special memory

    let mut name = String::from("John");
    println!("name before : {}",name);
    name.push_str(" Doe");
    println!("name after : {}",name);

    //TUPPLE
    let emp_info = ("John", 25, "Software Engineer");
    println!("Name: {} Age: {} Role: {}", emp_info.0, emp_info.1, emp_info.2);

    //DESTRUCTURING TUPPLE

    let (Ename,Eage,Erole) = emp_info;
    println!("Name: {} Age: {} Role: {}", Ename, Eage, Erole);
}

