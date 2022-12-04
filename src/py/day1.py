def solve_part2() -> int:
    data = open("input/2022/day1.txt").read().strip().split("\n\n")
    elf_calories = []

    for elf in data:
        cal_sum = 0
        for line in elf.split():
            cal_sum += int(line.strip())
        elf_calories.append(cal_sum)
    elf_calories = sorted(elf_calories, reverse=True)
    return sum(elf_calories[:3])

def test_solver():
    print(solve_part2())