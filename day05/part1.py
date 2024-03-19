class Range:
    def __init__(self, src_start, dest_start, len):
        self.src_start = src_start
        self.dest_start = dest_start
        self.len = len

    def contains(self, x) -> bool:
        return self.src_start <= x and x < self.src_start + self.len

    def map(self, x):
        if not self.contains(x):
            return None
        return self.dest_start + (x - self.src_start)

class Map:
    def __init__(self, ranges):
        self.ranges = ranges 

    def map(self, src):
        for _range in self.ranges:
            dest = _range.map(src)
            if dest is not None:
                return dest
        return src

    def append(self, _range: Range):
        self.ranges.append(_range)


def parse_map(lines) -> Map:
    map = Map([])
    for line in lines:
        if ':' in line or line == '':
            return map

        dest_start, src_start, len = [int(num) for num in line.split(' ')]
        map.append(Range(src_start, dest_start, len))

    return map

def process(lines) -> int:
    lines = (line.strip() for line in lines if line.strip())
    seeds = next(lines).split(':')[1]
    seeds = [int(num) for num in seeds.strip().split(' ')]
    next(lines)

    seed_to_soil            = parse_map(lines)
    soil_to_fertilizer      = parse_map(lines)
    fertilizer_to_water     = parse_map(lines)
    water_to_light          = parse_map(lines)
    light_to_temperature    = parse_map(lines)
    temperature_to_humidity = parse_map(lines)
    humidity_to_location    = parse_map(lines)

    soils       = [seed_to_soil.map(seed) for seed in seeds]
    fertilizers = [soil_to_fertilizer.map(soil) for soil in soils]
    waters      = [fertilizer_to_water.map(fertilizer) for fertilizer in fertilizers]
    lights      = [water_to_light.map(water) for water in waters]
    temps       = [light_to_temperature.map(light) for light in lights]
    humidities  = [temperature_to_humidity.map(temps) for temps in temps]
    locations   = [humidity_to_location.map(humidity) for humidity in humidities]

    return min(locations)

def test():
    lines = """
    seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4
    """.split('\n')
    assert process(lines) == 35, "Failed test"

if __name__ == '__main__':
    test()
    with open('input.txt', 'r') as file:
        print(f'Result: {process(file)}')
