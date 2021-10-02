pub fn run(){

    // do binary search
    let mut v = vec![1,2,3,4,5,6,7,8,9,10];
    let mut index = 0;
    let mut found = false;
    let mut low = 0;
    let mut high = v.len() - 1;
    let mut mid = (low + high) / 2;

    while !found && low <= high {
        if v[mid] == 8 {
            found = true;
        } else if v[mid] < 8 {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
        mid = (low + high) / 2;
    }

    if found {
        println!("Found {} at index {}", v[mid], mid);

    } else {
        println!("Not found");
    }

}