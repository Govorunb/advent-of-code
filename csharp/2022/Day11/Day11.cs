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

string input = File.ReadAllText("Input.txt");

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
