fn main() {
    hello();
    data_types();
    function(5);
    let added_value_returned:i8 = funcftion_with_return( 5,6);
    println!("added_value_returned:{}",added_value_returned)
}
fn hello(){
    println!("Hello, world!");
}

fn data_types(){
    //INT signed:- i8,i16.... unsigned: u8,u16...
    
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

    let (ename,eage,erole) = emp_info;
    println!("Name: {} Age: {} Role: {}", ename, eage, erole);
}

fn function(item:i8){
    println!("item: {}", item);
}
fn funcftion_with_return(item1:i8,item2:i8)->i8{
    return item1+item2;
}