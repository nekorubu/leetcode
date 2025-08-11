import math
def productQueries(n: int, queries: list[list[int]]):
    answers = [1] * len(queries)
    powers = []
    ANSWER_LIMIT = (10**9) + 7
    
    power_start = math.floor(math.log(n, 2)) # Logarithm. I want a logarithm kekw. See here lol: https://www.reddit.com/r/learnpython/comments/7ga2ng/how_to_find_the_closest_power/
    for x in reversed(range(power_start+1)):
        original_n = n
        n = n % 2**x
        if (n != original_n):
            powers.append(2**x)
    else:
        powers.sort()
    for i in range(len(queries)):
        for j in range(queries[i][0], queries[i][1]+1):
            answers[i] = answers[i] * powers[j]
        else:
            answers[i] = answers[i] % ANSWER_LIMIT

def main():
    # Test case for debugging
    productQueries(15, [[0,1],[2,2],[0,3]])

main()
