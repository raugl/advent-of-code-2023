def process(lines) -> int:
    total_score = 0
    for line in lines:
        card_score = 0
        win_nums, our_nums = line.strip().split(':')[1].split('|')
        win_nums = [int(num) for num in win_nums.split(' ') if num]
        our_nums = [int(num) for num in our_nums.split(' ') if num]

        for num in our_nums:
            if num in win_nums:
                if card_score == 0:
                    card_score = 1
                else:
                    card_score *= 2

        total_score += card_score

    return total_score

def test():
    lines = """
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    """.split('\n')

    lines = (line for line in lines if line.strip())
    assert process(lines) == 13, "Failed test"

if __name__ == '__main__':
    test()
    with open('input.txt', 'r') as file:
        print(f'Result: {process(file)}')
