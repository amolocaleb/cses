//https://cses.fi/problemset/task/1083
pub fn find_missing(buf: &[u32]) -> Vec<u32> {
    let mut counter: u32 = 0;
    let length = *(buf.iter().max().unwrap());
    let mut missing: Vec<u32> = Vec::new();
    while counter < length  {
        counter +=1;
        if ! &buf.contains(&counter) {
            missing.push(counter); 
        }
        


    }
    return missing;
}