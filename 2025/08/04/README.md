# Day 1 of solving LeetCode problems
## [1. Two Sum](https://leetcode.com/problems/two-sum)

When I first started doing these, I decided to just start from the beginning of the problems and just work my way up from there 1 day at a time. Though it took me a little bit to figure out what to do to pass the test cases, I eventually got it done and felt pretty proud of myself. You can see my solution [here](./1.two-sum.rs).

I tried the daily question a bit afterwards.

## [904. Fruit Into Baskets](https://leetcode.com/problems/fruit-into-baskets/?envType=daily-question&envId=2025-08-04)

Something funny that happened with this was that, when I came up with a previous version of my solution, I was passing the tests. That is, nearly all of them. There was a single case at the very end that I wasn't passing because my code was taking too long to finish it.

You know what that test case was?

**99,997 zeroes**, followed by *a 1, 2, and a 3*.

I eventually solved it by just making loop short-circuit if the current number was the same as the previous one, but good grief...

[Here was my solution](./904.fruit-into-baskets.rs).
