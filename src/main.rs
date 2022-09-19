use std::fs::File;
use std::io::prelude::*;
use sha2::Sha256;
use sha2::Digest;


fn main(){

    println!("Thank you :)");
    
}

// Return end result.
fn result(filename:String) -> String{
    let mut vec: Vec<String> = Vec::new();
    for (index,line) in read_file(filename).lines().enumerate(){
        if index != 0{
            vec.push((hash_input(line)).to_string());
        }
    }
    while vec.len() > 1{
        vec = create_next_level(vec);
    }

    return vec[0].to_string();
}

// Helper function to open and read the file.
fn read_file(filename:String) -> String{
    let mut file = File::open(filename).expect("Can't open the file.");
    let mut content= String::new();
    file.read_to_string(&mut content).expect("Can't read the file.");
    return content;
}

// You can use templates below or just remove
// Helper function to create a next leaves level may help you :)
fn create_next_level(current_level: Vec::<String>) -> Vec::<String> {
    let mut new_vec:Vec<String> = Vec::new();
    for i in 0..current_level.len()/2{
        let string = (format!("{}{}",current_level[2*i],current_level[2*i+1])).to_string();
        new_vec.push(hash_input(&string));
    }
    return new_vec;
}


// Helper function may help you to hash an input or You can write macro rules
fn hash_input(a: &str) -> String {
    let mut hasher = Sha256::new();
    let input = a;
    hasher.update(input);
    let hash = hasher.finalize();
    let hex = hex::encode(&hash);

    return hex.to_string();
}

// Pass all tests!
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_0() {
        let result = result("input0.txt".to_string());
        assert_eq!(result, "ff41418be11ed77612aeb83ee0bcf97a5853a4c291e23bd4d4cc6435fcfabdf9");
    }

    #[test]
    fn input_1() {
        let result = result("input1.txt".to_string());
        assert_eq!(result, "98a77b2c3ff5f6c2aca697f60b2aa2a1a2733be36dbd35bae23d517c2ad5985e");
    }

    #[test]
    fn input_2() {
        let result = result("input2.txt".to_string());
        assert_eq!(result, "3c0fb0638de91551eae4e9d984d72034aa0693be37b51737e6b81bc489866e5e");
    }

    #[test]
    fn input_3() {
        let result = result("input3.txt".to_string());
        assert_eq!(result, "f03b1c9163babeb728ac011fe0c2c9c69700a2f8ddde211ec07d621cdb322cfe");
    }

    #[test]
    fn input_4() {
        let result = result("input4.txt".to_string());
        assert_eq!(result, "f83e74742fda659dfc07615881af796abafc434f591aeb23b9f4366abe03c597");
    }
}