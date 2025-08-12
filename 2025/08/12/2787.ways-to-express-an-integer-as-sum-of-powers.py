def numberOfWays(n: int, x: int):
    import math

    ANSWER_LIMIT = (10**9) + 7
    answers = 0

    start = math.floor(n**(1/x))

    for i in reversed(range(1, start+1)):
        # print("Trying i:", i)
        temp_n = n
        working_n = temp_n
        # print("n:", n)
        # print("temp_n:", n)
        # print("---")
        for j in reversed(range(1, i+1)):
            # print("temp_n:", temp_n)
            # print("working_n:", working_n)
            # print("---")
            # print("Trying j:", j)
            working_n = working_n - j**x

            # print("temp_n:", temp_n)
            # print("working_n:", working_n)
            # print("---")

            if (working_n <= 0):
                # print(j, "might work")
                # print("---")
                if (working_n < 0):
                    # print(j, "doesn't work. Reseting.")
                    # print("---")
                    working_n = working_n + j**x
                    # print("temp_n:", temp_n)
                    # print("working_n:", working_n)
                    # print("---")
                    # Stop, go to the next number
                    continue
                # print(j, "does work. Incrementing answers.")
                answers += 1
                # print("answers:", answers)
                # print("---")
                working_n = working_n + j**x
                continue
            else:
                j = working_n**(1/x)
                # print("j:", j)
    # print("answers:", answers)

numberOfWays(7, 1)
