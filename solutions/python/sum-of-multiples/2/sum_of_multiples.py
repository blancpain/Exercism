def sum_of_multiples(limit, multiples):
    unique_multiples = set()

    for multiple in multiples:
        current_multiples = [
            x for x in range(1, limit) if multiple != 0 and x % multiple == 0
        ]
        unique_multiples.update(current_multiples)

    return sum(unique_multiples)
