fn main() {
    let mut x = 5;
    { // create a scope
        let y = &mut x; 
        *y += 1;
    }
    { // create another scope
        let z = &mut x; 
        *z += 1;
    }
    println!("x = {}", x);
}