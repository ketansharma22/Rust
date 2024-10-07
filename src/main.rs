// fn main() {
//     println!("Hello, Ketan");
//     let mut sentence=0;
//     io(sentence);
// }
// fn io(mut sentence: i32){
//     for mut i in 20..40 {
//         if sentence+i % 2 ==0{
//             println!("{}",sentence+i);
//         }
//
//         i=i+1;
//     }
// }

// fn main(){
//     println!("helo");
//     let mut num=12;
//     fun(num);
// }
//
// fn fun(mut num: i32){
//     num=num+1;
//     println!("{}", num+1);
// }
//


// fn main(){
//     let num=12;
//     println!("{}",num);
//     // funcc("ketan");
//     update();
// }
//
// fn funcc(st: &str){
//     println!("{}",st);
// }
//
// fn update(){
//     let mut s:String =String::from("Hello");
//
//     for _ in 0..1100{
//         s.push_str("World");
//         println!("{},{},{:p}",s.capacity(),s.len(),s.as_ptr());
//
//     }
//
//



//ownership concept
// fn main(){
//     let mut s1=String:: from("first");
//     s1=ownership(s1);
//     println!("first owner,{}",s1);
// }
// fn ownership(s:String)->String{
//     println!(" second owner,{}",s);
//     return s;
// }


//borrowing and reference

fn main(){
    let mut s1=String::from("hello");
    let s2=&s1;
    println!("{}", s2);
    println!("{}",s1);
    s1="";
}

