impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        // The highest number of fruits that were able to be collected
        let mut max_fruit = 0;
        
        // Loop to keep track of where we're at among the trees
        for x in 0..fruits.len() {
            // Funny funny short-circuit to check if the current fruit type is the same as the
            // previous one lol
            if x != 0 && fruits[x] == fruits[x-1] {
                continue;
            }

            // A collection of the types of fruits collected so far
            let mut fruit_types: Vec<i32> = Vec::with_capacity(2);
            
            // The number of fruit collected
            let mut fruit: i32 = 0;
            
            // Loop to keep track of the trees to try and collect from
            for y in x..fruits.len() {
                // Check to see if this fruit is one of the previous ones
                // NOTE: Using an iterator to access this `any()` function to check to see if
                // the fruit matches one of the ones we've collected. Looked up how to do this
                // to avoid having to use yet another for loop to search for it.
                // Yes, it would've been a small one, but I did want to make this any more
                // complicated than it already felt...
                if fruit_types.iter().any(|&z| z == fruits[y]) {
                    // If so, take the fruit
                    fruit += 1;
                }
                // Check to see if we're out of baskets of infinite holding
                else if !(fruit_types.len() == fruit_types.capacity()) {
                    // If we're not out of baskets, toss it into an empty basket
                    fruit_types.push(fruits[y]);
                    fruit += 1;
                }
                // If neither are true, then stop collecting
                else {
                    break;
                }
            }
            // Once we're done, check to see if we've reached a new record on maximum fruit
            // colected
            if fruit > max_fruit {
                max_fruit = fruit;
            }
        }

        // Return the highest number of fruit collected
        max_fruit
    }
}
