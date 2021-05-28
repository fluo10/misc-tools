use std::env;

//fn readline(&mut src: String, &mut dst: String)-> Result() {
//    println!("Please input source directory.");
//    io::stdin()
//        .read_line(&mut src)
//        .expect("Failed to read line");
//    println!("Please input source directory.");
//                
//    io::stdin()
//        .read_line(&mut dst)
//        .expect("Failed to read line");
//}
fn validate_args() {

}

fn main() {
    //let &mut src;
    //let &mut dst;

    let args: Vec<String> = env::args().collect();
    let src = &args[1];
    let dst = &args[2];
    
    println!("src: {}", src);
    println!("dst: {}", dst);
}
