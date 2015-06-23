fn main(){
    let v = vec![1,2,3];
    for i in &v{
        println!("vector: {}", i);
    }
    let a = from_slice(&v);
    for i in 0..a.len(){
        println!("array: {}", a[i]);
    }
}

fn from_slice(vector: &Vec<u8>) -> [u8; 10] {
   let mut a = [0; 10];
   for i in 0..vector.len() {
      a[i] = vector[i];
   }
    a
}
