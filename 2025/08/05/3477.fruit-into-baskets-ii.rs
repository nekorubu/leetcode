impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        // The amount of fruit in each basket. Using to keep track of if there's anything in the
        // basket
        let mut basket_capacity: Vec<i32> = vec![0; baskets.len()];
        // The number of unplaced fruits
        let mut unplaced_fruits = 0;

        for x in 0..fruits.len() {
            // Tracker for if the fruit has been placed in a basket yet
            let mut placed = false;
            // Loop for checking against the basket capacities
            for y in 0..baskets.len() {
                // Check if the basket is empty, as well as if it has enough space for the fruit
                if (basket_capacity[y] == 0) && (baskets[y] >= fruits[x]) {
                    // If so, place it in there, mark that it's been placed, and stop looking
                    basket_capacity[y] = fruits[x];
                    placed = true;
                    break;
                }
            }
            
            // If the fruit hasn't been placed, increase the tally
            if !placed {
                unplaced_fruits += 1;
            }
        }

        // Return the number of unplaced fruits
        unplaced_fruits
    }
}
