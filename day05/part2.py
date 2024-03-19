from collections import deque

class Range:
    def __init__(self, start, len):
        self.start = start
        self.len = len

    def contains(self, x) -> bool:
        return self.start <= x and x < self.start + self.len

    def intersect(self, other):
        print(f'Range::intersect: {type(other)}')
        start = max(self.start, other.start)
        end = min(self.start + self.len, other.start + other.len)
        if not start < end:
            return None

        _min = min(self.start, other.start)
        pre_diff = Range(_min, start - _min)

        _max = max(self.start + self.len, other.start + other.len)
        post_diff = Range(end, _max - end)

        diff = []
        if pre_diff.len != 0:
            diff.append(pre_diff)
        if post_diff.len != 0:
            diff.append(post_diff)

        return [Range(start, end - start), diff]

class MapRange:
    def __init__(self, src_start, dest_start, len):
        self.src_range = Range(src_start, len)
        self.dest_start = dest_start

    def map(self, other: Range):
        print(f'MapRange::map: {type(other)}')
        common_range = self.src_range.intersect(other)
        if common_range == None:
            return None

        common_range[0] = Range(self.dest_start + (common_range.start - self.src_range.start), common_range.len)
        return common_range

class Map:
    def __init__(self, ranges):
        self.ranges = ranges 

    def map(self, src: Range) -> [Range]:
        print(f'Map::map {type(src)}')
        result = []
        default = [src]
        srcs = deque([src])

        while len(srcs):
            src = srcs.popleft()
            for _range in self.ranges:
                dest = _range.map(src)
                if dest != None:
                    dest, diffs = dest
                    result.append(dest)
                    for diff in diffs:
                        srcs.append(diff)

        if not len(result):
            return default
        return result

    def append(self, _range: MapRange):
        self.ranges.append(_range)


def parse_map(lines) -> Map:
    map = Map([])
    for line in lines:
        if ':' in line or line == '':
            return map

        dest_start, src_start, len = [int(num) for num in line.split(' ')]
        map.append(MapRange(src_start, dest_start, len))

    return map

def process(lines) -> int:
    lines = (line.strip() for line in lines if line.strip())
    vals = next(lines).split(':')[1]
    vals = [int(num) for num in vals.strip().split(' ')]
    next(lines)

    seeds = []
    for i in range(0, len(vals), 2):
        seeds.append(Range(vals[i], vals[i + 1]))

    seed_to_soil            = parse_map(lines)
    soil_to_fertilizer      = parse_map(lines)
    fertilizer_to_water     = parse_map(lines)
    water_to_light          = parse_map(lines)
    light_to_temperature    = parse_map(lines)
    temperature_to_humidity = parse_map(lines)
    humidity_to_location    = parse_map(lines)

    soils = []
    for seed in seeds:
        soils.extend(seed_to_soil.map(seed))

    fertilizers = []
    for soil in soils:
        fertilizers.extend(soil_to_fertilizer.map(soil))

    waters = []
    for fertilizer in fertilizers:
        waters.extend(fertilizer_to_water.map(fertilizer))

    lights = []
    for water in waters:
        lights.extend(water_to_light.map(water))

    temperatures = []
    for light in lights:
        temperatures.extend(light_to_temperature.map(light))

    humidities = []
    for temperature in temperatures:
        humidities.extend(temperature_to_humidity.map(temperature))

    locations = []
    for humidity in humidities:
        locations.extend(humidity_to_location.map(humidity))

    return min([loc.start for loc in locations])

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
    assert process(lines) == 46, "Failed test"

if __name__ == '__main__':
    test()
    # with open('input.txt', 'r') as file:
    #     print(f'Result: {process(file)}')
