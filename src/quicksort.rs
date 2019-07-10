pub fn quicksort(vector: Vec<u32>) {
    // do the first split
    let depth_one = split_in_three(vector);
    println!("{:?}", depth_one);

    // Do the second split
    let mut deep = split_a_vector_of_vectors(depth_one);

    loop {
        // break the loop when the sorting is done
        if is_sorted(&deep) {
            break;
        }

        // perform the nth split
        deep = split_a_vector_of_vectors(deep);
        println!("{:?}", deep);
        thread::sleep(time::Duration::from_millis(200));
    }

    // Once sorted out, flattens the vector of vectors
    let mut sorted_one: Vec<u32> = Vec::new();
    for vector in deep.iter() {
        sorted_one.push(unwrap_the_vector(vector));
    }
    println!(
        "done! Here is your sorted vector all cleaned up: \n{:?} ",
        sorted_one
    );
}

// check if sorted
fn is_sorted(deep: &Vec<Vec<u32>>) -> bool {
    let length: u32 = deep.len() as u32;
    let mut number_of_singles: u32 = 0;

    for vector in deep.iter() {
        if vector.len() == 1 {
            number_of_singles += 1;
        }
    }
    length == number_of_singles
}

fn unwrap_the_vector(vector: &Vec<u32>) -> u32 {
    vector[0]
}

// this function takes ownership of the vector and consumes it - good
fn split_in_three(mut vector: Vec<u32>) -> Vec<Vec<u32>> {
    // Pop the pivot off the vector, put it in its own vector
    let pivot = vector.pop().unwrap();
    let mut pivot_vec: Vec<u32> = Vec::new();
    pivot_vec.push(pivot);

    let mut right_vec: Vec<u32> = Vec::new();
    let mut left_vec: Vec<u32> = Vec::new();

    // sort the vector
    for value in vector.iter() {
        if value > &pivot {
            right_vec.push(*value)
        } else {
            left_vec.push(*value)
        }
    }
    // the method to_vec() COPIES into a new vector
    [left_vec, pivot_vec, right_vec].to_vec()
}

// Takes ownership as well
fn split_a_vector_of_vectors(lvl2_vector: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    // create the super-vector that will be returned
    let mut new_lvl2_vector: Vec<Vec<u32>> = Vec::new();

    // iterate over the super-vector
    for lvl1_vector in lvl2_vector.iter() {
        // split each level 1 vector into three IF it's not empty
        if !lvl1_vector.is_empty() {
            let temp_lvl2_vector = split_in_three(lvl1_vector.to_vec());

            // push them to the returned lvl2 vector IF they're not empty
            for fresh_lvl1_vector in temp_lvl2_vector.iter() {
                if !fresh_lvl1_vector.is_empty() {
                    new_lvl2_vector.push(fresh_lvl1_vector.to_vec());
                }
            }
        }
    }
    new_lvl2_vector
}
