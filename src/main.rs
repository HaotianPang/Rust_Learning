use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {  
   ownship_return_function();
//函数返回值也会转移所有权
fn ownship_return_function(){
    let a=sub1_return_function();
    println!("now you get data {}",a);
    let b=sub2_return_function(a);
    println!("now you get the data back,and the data is:{}",b);
}
//返回值与所有权1
fn sub1_return_function()->String{
    let a =String::from("saifgq312");
    a
}
//返回值与所有权2
fn sub2_return_function(data:String)->String{
    println!("you get the data in second function:{}",data);
    data
}

//函数传参对变量所有权的影响
fn ownship_in_function(){
    let data=String::from("1231asda");
    println!("data ={}",data);
    sub_in_function(data);
    // println!("original data is {}",data);    data的变量已经被转移的，在此处使用会产生编译错误
}

//配合传参demo使用
fn sub_in_function(data:String){
    println!("sub function get data:{}",data);
}
//所有权转移demo
fn move_demo(){
    //存储在stack当中的元素“=”表达的是赋值
    let x=10;
    let y=x;
    println!("x is {}",x);
    println!("y is {}",y);

    //存储在heap（堆）中的元素，“=”转移了所有权，之前拥有的那个变量，现在没有对应的值了，不能被使用了
    let a=String::from("123123");
    let b=a;
    // println!("string a is {}",a);   这一行在ownship上的错误，会在编译的时候被挑出来
    println!("string b is {}",b);
}

fn string_use(){
    let mut strig=String::from("sdif23123");
    strig.push_str(",here is appended data");
    println!("string is {}",strig);
}

fn contro_flow(x:i32){
    if x>5{
        println!("x is bigger than 5");
    }else{
        println!("x is not bigger than 5");
    }

    //因为if是一个表达式，表达式会返回值，所以可以用在let后面进行赋值
    //但是用这种方法的时候，要保证所有的变量类型是相同的，因为在编译的时候要能够确定变量的类型
    let b=if x>5{1000}else{0};
    println!("b is {}",b);
    //loop会已知循环，指导嵌入ctrl+c或者是break关键字被执行；
    //但是其本身是一个表达式，也可以用来赋值
    //关键是要记住，赋值，一定不论表达式执行结果怎么样，一定要有一个值被返回
    let mut count=0;
    let c=loop{
        count+=1;
        if count==100{
            break count
        }
    };
    println!("c is {}",c);
    //while用例
    count=0;
    while count!=100{
        count+=1
    };
    println!("{}",count);

    //for用例
    println!("here is \"for\"");
    let a=[1,2,3,4,5];
    for number in a.iter(){
        println!("a is {}",number);
    }
    //单纯的range类型不需要使用括号，直接in就可以了
    for data in 1..10 {
        println!("data is {}",data);
    }
    println!("and here is reverse version");
    for data in (1..10).rev(){
        println!("{}",data);
    }
}

fn guess_game(){
    let secret_number=rand::thread_rng().gen_range(1,101);
    println!("the secret number is:{}",secret_number);
    println!("let's play a guessing game!");
    println!("please input your number here!");
    let mut guess=String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("faild to get an input");
    println!("your guess is {}",guess);
    let guess:u32= match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>0,
    };

    match guess.cmp(&secret_number){
        Ordering::Greater=>println!("Too big"),
        Ordering::Equal=>println!("They are equal"),
        Ordering::Less=>println!("Too small"),
    }

}

fn use_tuple(){
 //tuple用例
    let tup:(i32,u64,char)=(1,123,'c');
    //用法1
    let (x,y,z)=tup;
    println!("x is {},y is {},z is {}.",x,y,z);
    //用法2
    let x2=tup.0;
    let y2=tup.1;
    let z2=tup.2;
    println!("x is {},y is {},z is {}.",x2,y2,z2);
}

fn use_array(){
    //数组用例
    let a=[1,2,3,4,5,6,7];
    let b:[i32;5]=[1,2,3,4,5];
    let mut c:[i32;5]=[0,0,0,0,0];
    c[0]=1;
    println!("{}",c[0]);
    let d=[3;5];//5个3
}