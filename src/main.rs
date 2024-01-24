use std::time::Instant;
use druid::widget::{Align, Flex, Label};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Binary Search Algorithm");

#[derive(Clone, Data, Lens)]
struct HelloState {
    label_array1: String, // Change label_array1 to a String
}

// Implement the binary search algorithm.
// Function to perform binary search on a sorted array.
fn binary_search(
    result: Result<usize, usize>,
    mut low: usize,
    mut high: usize,
    target: i32,
    array1: &[i32], // The sorted array to search.
    state: &mut HelloState, // Pass the HelloState to update label_array1
) {
    // Validate and obtain the result from the previous search operation.
    let result: Result<usize, usize> = result;

    // Execute binary search until the target is found or the search space is exhausted.
    while low <= high {
        // Calculate the middle index.
        let mid: usize = (low + high) / 2;
        // Retrieve the value at the middle index.
        let guess: i32 = array1[mid];

        let label_array: String = "[1..=100]".into();

        // Check if the guess is equal to the target.
        if guess == target {
            // Update label_array1 with search information for GUI display
            state.label_array1 = format!(
                "Search Results:\n----------------\narray1: {:?}\nmid: {}\nlow: {}\nhigh: {}\ntarget: {}\nguess: {}\nresult: {:?}\nDid I get it right? : {}",
                label_array, mid, low, high, target, guess, result, guess == target, 
            );
            // Exit the loop as the target is found.
            break;
        } else if guess > target {
            // Adjust the search space to the left.
            high = mid - 1;
        } else {
            // Adjust the search space to the right.
            low = mid + 1;
        }
    }
}

// Main function where the binary search algorithm is demonstrated.
fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // Sleep for 1 second for demonstration purposes.
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Define a sorted array of integers.
    let array1: [i32; 100] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
        27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
        51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74,
        75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98,
        99, 100];

    // Specify the target integer to search for.
    let target = 20;

    // create the initial app state
    let mut initial_state = HelloState {
        label_array1: String::new(),
    };

    // Perform binary search on the array to find the target.
    let result: Result<usize, usize> = array1.binary_search(&target);

    // Call the binary search function with the search result and other parameters.
    binary_search(result, 0, array1.len(), target, &array1, &mut initial_state);

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");

    // Implement benchmarking to check the speed of the algorithm based on time.

    let now: Instant = Instant::now();
    let elapsed = now.elapsed();
    
    println!("Benchmark Results:");
    println!("------------------");
    println!("Elapsed: {:.2?}", elapsed);
    println!("       : {:.10?} secs", elapsed.as_secs_f32());
    

    


    // press key to exit
    println!("Press any key to exit...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

fn build_root_widget() -> impl Widget<HelloState> {
    
    // a label that displays array1 information.
    let lab_array = Label::new(|data: &HelloState, _env: &Env| format!("Array {}!", &data.label_array1));

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(lab_array)
        .with_spacer(VERTICAL_WIDGET_SPACING);

    // center the two widgets in the available space
    Align::centered(layout)
}
