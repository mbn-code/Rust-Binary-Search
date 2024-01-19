// import visual libraries to show results

fn binary_search(
    result: Result<usize, usize>,
    mut low: usize,
    mut high: usize,
    target: i32,
    array1: [i32; 10]){

    let result: Result<usize, usize> = result;    
    while low <= high {
        let mid: usize = (low + high) / 2;
        let guess: i32 = array1[mid];
        if guess == target {
            print!("array1: {:?}\n", array1);
            print!("mid: {}\n", mid);
            print!("low: {}\n", low);
            print!("high: {}\n", high);
            print!("target: {}\n", target);
            print!("guess: {}\n", guess);
            print!("result: {:?}\n", result);

            print!("Did I get it right? : {}\n", guess == target);

            std::thread::sleep(std::time::Duration::from_secs(10));

            break;
        } else if guess > target {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
}


fn main() {
    println!("Hello, world!");

    // sleep 5 sec
    std::thread::sleep(std::time::Duration::from_secs(1));

    for i in 0..10 {
        println!("Yo This is: {}", i);
    }

    let array1: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let target = 5;

    let result: Result<usize, usize> = array1.binary_search(&target);

    print!("array1: {:?}\n", array1);

    binary_search(result, 0, array1.len(), target, array1);

    std::thread::sleep(std::time::Duration::from_secs(1));
}






