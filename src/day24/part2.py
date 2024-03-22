from typing import Tuple

def process(lines, bounds: Tuple[int, int]) -> int:
    # Collide all rays with an 3D plane
    # Fit a straight line too all those point
    # Now using the normal to that line inside the plane, split the inetersections in two groups
    # based on the sign of the dot product of the direction of each ray and the normal
    # Now fit a straight line to each one of these two sets of points
    # Now advance the plane until the distance between the two lines is zero using binary search
    # by reapeating all the previous steps
    # Now rotate the plane until the distance between the two lines is zero using binary search
    # by reapeating all the previous steps. Use as the rotation axis the normal to the mid line
    # Get the intersection positions of all the rays with the stone axis and sort them based on the
    # time to collision.
    # Get the min stride between the collision points, that is the rock's velocity
    # To get the starting position, subtract the one velocity from the collision position
    # of the first hail stone to be hit


if __name__ == '__main__':
    with open('input.txt', 'r') as file:
        result = process(file, (200000000000000, 400000000000000))
        print(f'Result {result}')

