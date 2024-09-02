
// https://cses.fi/problemset/task/1068/
pub fn weird_algo(input: u32) {
    let mut num = input;
    
     
    let mut output: Vec<u32> = Vec::new();
    output.push(*(&num));
    let mask = 1 << 0;//even number test mask
    
    while num > 1 {
        let even_odd = &num & mask;
        if even_odd != 0 {
            num = (num*3)+1;
            output.push(*(&num)) 
        } else {
            num = (num)/2;
            output.push(*(&num))
        }
    }

    println!("{:?}",output.iter().map(|e|e.to_string()).collect::<Vec<_>>().join(" -> "));
    
}