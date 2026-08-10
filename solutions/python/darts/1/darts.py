def score(x: float, y: float) -> int:
    calc: float = pow(x, 2) + pow(y, 2)

    if calc <= 1**2:
        return 10
    if calc <= 5**2:
        return 5
    if calc <= 10**2:
        return 1

    return 0
