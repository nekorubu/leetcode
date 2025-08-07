impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        // The amount of fruit in each basket. Using to keep track of if there's anything in the
        // basket
        let mut basket_capacity: Vec<i32> = vec![0; baskets.len()];
        // The number of unplaced fruits
        let mut unplaced_fruits = 0;

        let mut baskets_sort: Vec<i32> = baskets.clone();
        baskets_sort.sort();

        for x in 0..fruits.len() {
            // Tracker for if the fruit has been placed in a basket yet
            let mut placed = false;

            let basket_index = baskets_sort.binary_search(&fruits[x]);

            let basket_index = match basket_index {
                Result::Ok(_) => basket_index.unwrap(),
                Result::Err(_) => basket_index.unwrap_err(),
            };

            if (basket_capacity[basket_index as usize] == 0) && (baskets_sort[basket_index as usize] >= fruits[x]) {
                basket_capacity[basket_index as usize] = fruits[x];
                placed = true;
                continue;
            } else {
                for y in (basket_index+1)..(basket_capacity.len()) {
                    if (basket_capacity[y] == 0) && (baskets_sort[y] >= fruits[x]) {
                        basket_capacity[y] = fruits[x];
                        placed = true;
                        break;
                    }
                }

                if !placed {
                    unplaced_fruits += 1;
                }
            }
        }

        // Return the number of unplaced fruits
        unplaced_fruits
    }
}
