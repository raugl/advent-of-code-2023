from typing import Tuple

class Vec2:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

    def check_bounds(self, bounds: Tuple[int, int]) -> bool:
        return bounds[0] <= self.x and self.x < bounds[1] and bounds[0] <= self.y and self.y < bounds[1]

class Ray:
    def __init__(self, origin: Vec2, dir: Vec2):
        self.origin = origin
        self.dir = dir

    def get_point(self, t: float) -> Vec2:
        return Vec2(self.origin.x + self.dir.x * t, self.origin.y + self.dir.y * t)

def intersect_rays(ray1: Ray, ray2: Ray) -> Tuple[float, float]:
    o1, d1 = (ray1.origin, ray1.dir)
    o2, d2 = (ray2.origin, ray2.dir)

    div = d1.y * d2.x - d1.x * d2.y
    if div == 0:
        return (0, 0)

    t2 = (d1.x * (o2.y - o1.y) - d1.y * (o2.x - o1.x)) / div
    t1 = (o2.x - o1.x + d2.x * t2) / d1.x
    return (t1, t2)

def process(lines, bounds: Tuple[int, int]) -> int:
    rays = []
    for line in lines:
        pos, vel = line.strip().split('@')
        pos = [int(num) for num in pos.split(',')[:-1]]
        vel = [int(num) for num in vel.split(',')[:-1]]
        rays.append(Ray(Vec2(pos[0], pos[1]), Vec2(vel[0], vel[1])))

    intersection_count = 0
    for ray1 in rays:
        for ray2 in rays:
            if ray1 is ray2:
                break

            t1, t2 = intersect_rays(ray1, ray2)
            if t1 > 0 and t2 > 0:
                intersection = ray1.get_point(t1)
                if intersection.check_bounds(bounds):
                    intersection_count += 1

    return intersection_count

if __name__ == '__main__':
    with open('input.txt', 'r') as file:
        result = process(file, (200000000000000, 400000000000000))
        print(f'Result {result}')

