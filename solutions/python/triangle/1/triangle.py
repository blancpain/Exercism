def equilateral(sides):
    return isValidTriangle(sides) and len(set(sides)) == 1


def isosceles(sides):
    return isValidTriangle(sides) and len(set(sides)) == 2 or len(set(sides)) == 1


def scalene(sides):
    return isValidTriangle(sides) and len(set(sides)) == 3


def isValidTriangle(sides) -> bool:
    if sum(sides) == 0:
        return False
    if sides[0] + sides[1] <= sides[2]:
        return False
    if sides[1] + sides[2] <= sides[0]:
        return False
    if sides[0] + sides[2] <= sides[1]:
        return False

    return True
