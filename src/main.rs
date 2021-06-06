use std::env;
use std::fs;
use std::path::Path;
mod svg_sprite;
use svg_sprite::SvgSprite;
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
    assert!(args.len() > 1);
    let src = &args[1];
    let src = Path::new(src);
    assert!(src.exists());
    let dst = match args.len() {
        2 => format!("{}{}",args[1], ".svg"),
        3 => format!("{}",args[2]),
        _ => panic!("Invalid arguments")
    };

    let dst = Path::new(&dst);
    assert!(src.exists());
    //let contents = fs::read_to_string(filename)
    println!("src: {}", src.display());
    println!("dst: {}", dst.display());
    //get_source_files(src);
    let mut svgs = SvgSprite::new();
    if src.is_dir() {
        for entry in  fs::read_dir(src).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                //println!("{}", path.as_path().display());
                //println!("{}", fs::read_to_string(path).unwrap());
                svgs.add_svg(&path);
            }
        }
        svgs.export(&dst);
    } else{
        println!("{} is not direcroty", src.display());
    }
}


//fn get_source_files(dir: &Path, cb: &dyn FN()) -> io::Result<()> {
//}