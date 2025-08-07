impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // nums.len() returns a usize. It can be used for indices, but when returning a vector, they have to be converted to integers (i32) using `try_into()` and `unwrap()` (see line 10)
        for x in 0..nums.len() {
            for y in 0..nums.len() {
                if y == x {
                    // Skip over checking against the same number to comply with the constraint
                    continue;
                } else if nums[y] == target - nums[x] {
                    return vec![x.try_into().unwrap(), y.try_into().unwrap()];
                }
            }
        }
        // In the event that solution isn't found in `nums`, return back a vector with [-1, -1]
        return vec![-1, -1];

        // No idea if this is O(n^2) time or not lol
    }
}
