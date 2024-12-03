global using AoC2022.Common;
using Day11;
using System.Diagnostics;

string testInput = """
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
""";

int p1TestOutput = 10605;

string input = """
Monkey 0:
  Starting items: 98, 97, 98, 55, 56, 72
  Operation: new = old * 13
  Test: divisible by 11
    If true: throw to monkey 4
    If false: throw to monkey 7

Monkey 1:
  Starting items: 73, 99, 55, 54, 88, 50, 55
  Operation: new = old + 4
  Test: divisible by 17
    If true: throw to monkey 2
    If false: throw to monkey 6

Monkey 2:
  Starting items: 67, 98
  Operation: new = old * 11
  Test: divisible by 5
    If true: throw to monkey 6
    If false: throw to monkey 5

Monkey 3:
  Starting items: 82, 91, 92, 53, 99
  Operation: new = old + 8
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 2

Monkey 4:
  Starting items: 52, 62, 94, 96, 52, 87, 53, 60
  Operation: new = old * old
  Test: divisible by 19
    If true: throw to monkey 3
    If false: throw to monkey 1

Monkey 5:
  Starting items: 94, 80, 84, 79
  Operation: new = old + 5
  Test: divisible by 2
    If true: throw to monkey 7
    If false: throw to monkey 0

Monkey 6:
  Starting items: 89
  Operation: new = old + 1
  Test: divisible by 3
    If true: throw to monkey 0
    If false: throw to monkey 5

Monkey 7:
  Starting items: 70, 59, 63
  Operation: new = old + 3
  Test: divisible by 7
    If true: throw to monkey 4
    If false: throw to monkey 3
""";

// common function defs
void SetupMonkeys(string input) {
    Monkey.Monkeys.Clear();
    var monkeyDefs = input.Split(StringExtensions.DoubleNewLine);
    foreach (var monkeyDef in monkeyDefs) {
        var monkey = Monkey.FromTextDef(monkeyDef);
    }
}


// part one
int PartOne(string input, bool debugOutput = false) {
    SetupMonkeys(input);
    if (Monkey.Monkeys.Count == 0)
        return -1;
    for (var i = 0; i < 20; i++) {
        foreach (var monkey in Monkey.Monkeys)
            monkey.Turn(true);
        if (debugOutput) {
            Console.WriteLine(
                string.Join("\n\t",
                    Monkey.Monkeys.Select(monkey => {
                        return $"Monkey {monkey.Index}: {string.Join(", ", monkey.Items)}";
                    }).Prepend($"After round {i + 1}:")
                )
            );
        }
    }
    if (debugOutput)
        Console.WriteLine('\n' + string.Join('\n', Monkey.Monkeys.Select(monkey => $"Monkey {monkey.Index}: {monkey.ItemsInspected}")));

    var topMonkeys = Monkey.Monkeys.OrderByDescending(monkey => monkey.ItemsInspected);
    var monkeyBusiness = topMonkeys.Take(2).Aggregate(1, (acc, monkey) => acc * monkey.ItemsInspected);
    return monkeyBusiness;
}

Debug.Assert(PartOne(testInput) == p1TestOutput);

Console.WriteLine(PartOne(input));

// part two

long p2TestOutput = 2713310158L;

long PartTwo(string input, bool debugOutput = false) {
    SetupMonkeys(input);
    if (Monkey.Monkeys.Count == 0)
        return -1;
    for (var i = 0; i < 10000; i++) {
        foreach (var monkey in Monkey.Monkeys)
            monkey.Turn(false);
        if (debugOutput && (i + 1) % 1000 == 0) {
            Console.WriteLine('\n' + $"After round {i + 1}:\n\t"
                + string.Join("\n\t",
                    Monkey.Monkeys.Select(monkey => {
                        return $"Monkey {monkey.Index}: {string.Join(", ", monkey.Items)}";
                    })
                )
            );
            Console.WriteLine(string.Join('\n', Monkey.Monkeys.Select(monkey => $"Monkey {monkey.Index} inspected items {monkey.ItemsInspected} times.")));
        }
    }
    if (debugOutput)
        Console.WriteLine("\nTotal:\n" + string.Join('\n', Monkey.Monkeys.Select(monkey => $"Monkey {monkey.Index}: {monkey.ItemsInspected}")));

    var topMonkeys = Monkey.Monkeys.OrderByDescending(monkey => monkey.ItemsInspected);
    var monkeyBusiness = topMonkeys.Take(2).Aggregate(1L, (acc, monkey) => acc * monkey.ItemsInspected);
    return monkeyBusiness;
}

Debug.Assert(PartTwo(testInput, true) == p2TestOutput);

Console.WriteLine(PartTwo(input, true));
