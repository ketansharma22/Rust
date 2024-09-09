fn main() {
    println!("Hello, Ketan");
    let mut sentence=0;
    io(sentence);
}
fn io(mut sentence: i32){
    for mut i in 20..40 {
        if sentence+i % 2 ==0{
            println!("{}",sentence+i);
        }

        i=i+1;
    }
}
