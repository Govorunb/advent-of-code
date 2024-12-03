// See https://aka.ms/new-console-template for more information

using System.Text.RegularExpressions;

(int count, int from, int to) EncodeInstruction(string instructionLine) {
    Regex pattern = InstructionLinePattern();
    var match = pattern.Match(instructionLine);
    return (int.Parse(match.Groups["count"].ValueSpan), int.Parse(match.Groups["from"].ValueSpan) - 1, int.Parse(match.Groups["to"].ValueSpan) - 1);
}

var input = """
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
""";

input = """
                    [Q]     [P] [P]
                [G] [V] [S] [Z] [F]
            [W] [V] [F] [Z] [W] [Q]
        [V] [T] [N] [J] [W] [B] [W]
    [Z] [L] [V] [B] [C] [R] [N] [M]
[C] [W] [R] [H] [H] [P] [T] [M] [B]
[Q] [Q] [M] [Z] [Z] [N] [G] [G] [J]
[B] [R] [B] [C] [D] [H] [D] [C] [N]
 1   2   3   4   5   6   7   8   9 

move 3 from 6 to 2
move 5 from 6 to 7
move 6 from 2 to 5
move 1 from 9 to 7
move 1 from 1 to 9
move 1 from 5 to 3
move 1 from 2 to 5
move 3 from 4 to 5
move 10 from 7 to 3
move 1 from 4 to 9
move 6 from 8 to 7
move 4 from 7 to 8
move 1 from 7 to 3
move 1 from 1 to 2
move 1 from 2 to 8
move 1 from 9 to 1
move 3 from 9 to 4
move 4 from 8 to 3
move 4 from 7 to 1
move 4 from 4 to 6
move 2 from 8 to 7
move 9 from 3 to 8
move 2 from 7 to 4
move 3 from 4 to 9
move 4 from 1 to 9
move 4 from 3 to 9
move 2 from 1 to 4
move 1 from 4 to 6
move 3 from 3 to 2
move 1 from 2 to 8
move 1 from 2 to 7
move 3 from 6 to 2
move 2 from 6 to 7
move 4 from 2 to 3
move 3 from 7 to 9
move 2 from 5 to 6
move 15 from 9 to 4
move 4 from 9 to 2
move 12 from 5 to 4
move 9 from 8 to 5
move 25 from 4 to 7
move 1 from 4 to 7
move 1 from 4 to 8
move 2 from 2 to 5
move 1 from 4 to 2
move 23 from 7 to 6
move 2 from 5 to 2
move 22 from 6 to 8
move 4 from 5 to 9
move 1 from 7 to 9
move 2 from 6 to 4
move 2 from 4 to 7
move 25 from 8 to 3
move 1 from 2 to 1
move 3 from 2 to 3
move 1 from 6 to 8
move 1 from 1 to 8
move 1 from 2 to 8
move 1 from 8 to 1
move 4 from 5 to 7
move 1 from 8 to 4
move 5 from 9 to 8
move 5 from 8 to 9
move 1 from 8 to 5
move 3 from 5 to 4
move 3 from 9 to 1
move 30 from 3 to 4
move 3 from 1 to 4
move 2 from 9 to 5
move 4 from 7 to 9
move 16 from 4 to 8
move 6 from 3 to 9
move 3 from 7 to 3
move 19 from 4 to 7
move 8 from 9 to 4
move 1 from 1 to 9
move 13 from 7 to 9
move 3 from 7 to 8
move 3 from 5 to 9
move 4 from 8 to 3
move 2 from 7 to 3
move 14 from 9 to 4
move 10 from 3 to 1
move 12 from 4 to 8
move 6 from 1 to 9
move 1 from 1 to 2
move 1 from 7 to 1
move 6 from 9 to 3
move 17 from 8 to 6
move 10 from 8 to 5
move 1 from 7 to 8
move 1 from 9 to 5
move 2 from 3 to 1
move 4 from 5 to 9
move 1 from 8 to 7
move 6 from 9 to 7
move 4 from 4 to 2
move 3 from 4 to 6
move 4 from 5 to 9
move 4 from 9 to 3
move 1 from 2 to 4
move 4 from 4 to 7
move 3 from 5 to 3
move 1 from 4 to 5
move 5 from 1 to 2
move 1 from 1 to 9
move 7 from 2 to 7
move 1 from 5 to 7
move 8 from 3 to 5
move 20 from 6 to 7
move 9 from 7 to 9
move 2 from 2 to 9
move 2 from 3 to 1
move 2 from 1 to 3
move 2 from 3 to 4
move 2 from 4 to 6
move 1 from 3 to 9
move 1 from 4 to 9
move 1 from 6 to 9
move 2 from 5 to 8
move 2 from 8 to 5
move 1 from 6 to 7
move 2 from 5 to 8
move 6 from 9 to 5
move 2 from 8 to 6
move 11 from 9 to 2
move 1 from 6 to 5
move 11 from 2 to 5
move 1 from 6 to 4
move 7 from 5 to 9
move 7 from 9 to 1
move 1 from 4 to 9
move 28 from 7 to 5
move 1 from 7 to 5
move 5 from 5 to 9
move 5 from 9 to 3
move 6 from 1 to 8
move 1 from 1 to 7
move 5 from 3 to 2
move 1 from 7 to 8
move 7 from 8 to 1
move 1 from 9 to 4
move 2 from 2 to 5
move 22 from 5 to 3
move 1 from 7 to 8
move 1 from 4 to 7
move 1 from 8 to 9
move 1 from 9 to 4
move 14 from 5 to 7
move 5 from 5 to 9
move 19 from 3 to 4
move 1 from 2 to 9
move 2 from 2 to 5
move 1 from 5 to 1
move 6 from 1 to 7
move 2 from 7 to 6
move 1 from 1 to 9
move 2 from 5 to 8
move 8 from 4 to 5
move 3 from 4 to 7
move 3 from 3 to 5
move 2 from 8 to 9
move 16 from 7 to 5
move 9 from 4 to 6
move 22 from 5 to 3
move 1 from 5 to 8
move 1 from 8 to 7
move 10 from 3 to 4
move 1 from 5 to 4
move 10 from 4 to 5
move 8 from 5 to 2
move 5 from 2 to 7
move 5 from 7 to 1
move 4 from 7 to 6
move 3 from 9 to 7
move 2 from 2 to 3
move 3 from 5 to 1
move 6 from 9 to 7
move 5 from 7 to 8
move 6 from 1 to 5
move 6 from 3 to 4
move 4 from 4 to 2
move 1 from 4 to 6
move 5 from 8 to 7
move 3 from 2 to 3
move 1 from 1 to 4
move 1 from 1 to 9
move 2 from 2 to 1
move 2 from 4 to 3
move 4 from 3 to 7
move 3 from 7 to 3
move 13 from 6 to 1
move 1 from 9 to 2
move 6 from 3 to 5
move 8 from 1 to 4
move 1 from 2 to 7
move 9 from 4 to 9
move 7 from 5 to 1
move 2 from 5 to 6
move 1 from 1 to 4
move 1 from 4 to 3
move 2 from 1 to 2
move 5 from 3 to 6
move 2 from 6 to 1
move 13 from 7 to 6
move 2 from 3 to 4
move 2 from 2 to 9
move 2 from 7 to 8
move 6 from 9 to 2
move 1 from 9 to 3
move 1 from 5 to 2
move 7 from 1 to 2
move 1 from 6 to 7
move 1 from 4 to 8
move 1 from 3 to 1
move 1 from 7 to 8
move 7 from 1 to 9
move 4 from 8 to 6
move 1 from 5 to 3
move 9 from 9 to 5
move 1 from 1 to 2
move 14 from 2 to 7
move 2 from 9 to 3
move 13 from 5 to 3
move 24 from 6 to 9
move 6 from 3 to 5
move 14 from 7 to 9
move 1 from 4 to 1
move 20 from 9 to 7
move 9 from 3 to 8
move 15 from 9 to 6
move 1 from 5 to 8
move 1 from 2 to 3
move 14 from 6 to 3
move 2 from 3 to 4
move 2 from 3 to 6
move 13 from 7 to 1
move 8 from 3 to 5
move 1 from 3 to 9
move 8 from 5 to 4
move 4 from 5 to 2
move 10 from 1 to 3
move 6 from 4 to 5
move 4 from 5 to 1
move 3 from 1 to 6
move 7 from 8 to 2
move 4 from 4 to 3
move 13 from 3 to 6
move 3 from 8 to 1
move 3 from 7 to 8
move 3 from 8 to 4
move 1 from 4 to 2
move 2 from 3 to 4
move 1 from 5 to 7
move 4 from 7 to 1
move 2 from 3 to 5
move 3 from 2 to 1
move 1 from 4 to 7
move 7 from 2 to 4
move 2 from 4 to 3
move 1 from 7 to 5
move 4 from 9 to 5
move 1 from 4 to 2
move 3 from 2 to 9
move 8 from 1 to 7
move 1 from 3 to 5
move 7 from 5 to 7
move 10 from 6 to 4
move 1 from 5 to 1
move 4 from 1 to 3
move 9 from 7 to 6
move 3 from 1 to 8
move 12 from 4 to 6
move 5 from 4 to 6
move 2 from 9 to 3
move 3 from 8 to 7
move 1 from 1 to 3
move 3 from 7 to 8
move 5 from 7 to 5
move 1 from 7 to 5
move 2 from 3 to 1
move 2 from 8 to 7
move 3 from 5 to 1
move 1 from 9 to 7
move 1 from 8 to 3
move 4 from 7 to 8
move 4 from 5 to 9
move 4 from 1 to 7
move 3 from 8 to 6
move 1 from 8 to 1
move 1 from 7 to 1
move 1 from 5 to 8
move 1 from 8 to 7
move 7 from 3 to 1
move 3 from 9 to 1
move 1 from 9 to 3
move 28 from 6 to 3
move 3 from 7 to 8
move 2 from 8 to 2
move 1 from 2 to 7
move 2 from 6 to 1
move 18 from 3 to 9
move 5 from 3 to 4
move 2 from 7 to 4
move 2 from 1 to 8
move 1 from 2 to 6
move 7 from 6 to 4
move 4 from 4 to 3
move 3 from 8 to 1
move 4 from 9 to 8
move 1 from 4 to 8
move 9 from 1 to 6
move 5 from 1 to 3
move 4 from 6 to 7
move 7 from 6 to 3
move 5 from 8 to 1
move 12 from 3 to 6
move 7 from 6 to 4
move 4 from 3 to 5
move 5 from 6 to 7
move 12 from 4 to 3
move 6 from 1 to 4
move 4 from 4 to 2
move 14 from 9 to 8
move 17 from 3 to 2
move 5 from 4 to 9
move 1 from 9 to 6
move 5 from 2 to 1
move 1 from 9 to 8
move 5 from 1 to 6
move 2 from 2 to 6
move 12 from 2 to 4
move 6 from 7 to 2
move 3 from 7 to 6
move 3 from 9 to 8
move 5 from 4 to 7
move 4 from 2 to 6
move 3 from 6 to 8
move 5 from 8 to 2
move 7 from 6 to 8
move 1 from 7 to 3
move 6 from 4 to 3
move 1 from 8 to 1
move 1 from 5 to 7
move 2 from 6 to 8
move 13 from 8 to 2
move 3 from 5 to 4
move 1 from 1 to 2
move 3 from 6 to 2
move 1 from 1 to 4
move 4 from 4 to 8
move 8 from 3 to 1
move 2 from 4 to 8
move 15 from 2 to 4
move 16 from 8 to 3
move 1 from 8 to 6
move 1 from 7 to 2
move 8 from 1 to 2
move 1 from 6 to 8
move 6 from 3 to 1
move 3 from 3 to 8
move 6 from 3 to 1
move 6 from 2 to 9
move 2 from 1 to 4
move 1 from 8 to 5
move 8 from 2 to 9
move 8 from 1 to 4
move 3 from 8 to 6
move 21 from 4 to 7
move 1 from 9 to 7
move 2 from 6 to 8
move 1 from 5 to 1
move 1 from 3 to 9
move 8 from 9 to 4
move 1 from 1 to 7
move 1 from 1 to 4
move 1 from 6 to 8
move 1 from 9 to 3
move 2 from 9 to 5
move 2 from 5 to 3
move 1 from 9 to 4
move 3 from 8 to 2
move 1 from 1 to 4
move 4 from 4 to 9
move 3 from 3 to 2
move 5 from 9 to 1
move 17 from 7 to 1
move 1 from 9 to 1
move 2 from 2 to 4
move 1 from 4 to 2
move 8 from 2 to 9
move 5 from 4 to 5
move 6 from 4 to 8
move 20 from 1 to 6
move 2 from 9 to 8
move 1 from 2 to 9
move 2 from 8 to 7
move 8 from 7 to 8
move 4 from 5 to 9
move 14 from 8 to 7
move 1 from 5 to 7
move 7 from 9 to 1
move 3 from 6 to 4
move 3 from 9 to 7
move 12 from 6 to 7
move 22 from 7 to 9
move 2 from 2 to 5
move 10 from 1 to 7
move 1 from 4 to 1
move 2 from 6 to 1
move 1 from 1 to 3
move 2 from 4 to 8
move 2 from 8 to 6
move 1 from 3 to 8
move 1 from 4 to 1
move 2 from 5 to 3
move 1 from 8 to 4
move 2 from 3 to 7
move 19 from 9 to 7
move 1 from 1 to 4
move 2 from 9 to 1
move 2 from 1 to 6
move 1 from 6 to 5
move 42 from 7 to 8
move 1 from 7 to 6
move 2 from 4 to 8
move 7 from 6 to 8
move 2 from 1 to 5
move 2 from 9 to 5
move 14 from 8 to 3
move 22 from 8 to 2
move 3 from 5 to 6
move 10 from 8 to 6
move 5 from 8 to 9
move 12 from 6 to 7
move 2 from 5 to 1
move 5 from 3 to 2
move 7 from 3 to 5
move 2 from 5 to 1
move 2 from 3 to 7
move 4 from 1 to 2
move 1 from 5 to 7
move 1 from 5 to 4
move 1 from 6 to 2
move 1 from 9 to 2
move 9 from 7 to 3
move 1 from 4 to 1
move 3 from 7 to 5
move 4 from 3 to 2
move 5 from 2 to 3
move 2 from 5 to 2
move 34 from 2 to 9
move 1 from 1 to 5
move 15 from 9 to 3
move 2 from 3 to 2
move 1 from 5 to 4
move 7 from 3 to 8
move 3 from 9 to 2
move 6 from 9 to 4
move 5 from 9 to 3
move 4 from 4 to 6
move 1 from 6 to 8
move 1 from 3 to 5
move 6 from 3 to 2
move 1 from 4 to 9
move 2 from 4 to 2
move 4 from 5 to 8
move 1 from 5 to 6
move 1 from 7 to 6
move 1 from 9 to 6
move 1 from 7 to 2
move 12 from 8 to 7
move 2 from 7 to 3
move 4 from 6 to 9
move 7 from 9 to 4
move 9 from 3 to 9
move 11 from 7 to 4
move 3 from 9 to 6
move 1 from 4 to 1
move 15 from 4 to 3
move 2 from 4 to 1
move 3 from 1 to 4
move 17 from 3 to 7
move 4 from 3 to 7
move 7 from 9 to 2
move 3 from 4 to 1
move 4 from 6 to 9
move 1 from 9 to 6
move 1 from 3 to 1
move 5 from 7 to 9
move 8 from 9 to 4
move 1 from 1 to 6
move 6 from 4 to 9
move 4 from 2 to 3
move 1 from 4 to 3
move 1 from 4 to 9
move 1 from 1 to 7
move 1 from 7 to 9
move 3 from 6 to 2
move 9 from 2 to 3
move 1 from 9 to 4
move 1 from 1 to 5
move 12 from 7 to 6
move 4 from 9 to 8
""";


var _tempSplit = input.Split(Environment.NewLine + Environment.NewLine);
(string initialStateString, string instructionsString) = (_tempSplit[0], _tempSplit[1]);

var initialStateLines = initialStateString.Split(Environment.NewLine);
var stackNumbers = initialStateLines.Last()
    .Split(' ', StringSplitOptions.RemoveEmptyEntries)
    .Select(int.Parse);

var numCrates = initialStateString.Count(c => c == '[');
var numStacks = stackNumbers.Last();

Stack<char>[] stacks = new Stack<char>[numStacks];
for (var i = 0; i < numStacks; i++)
    stacks[i] = new Stack<char>();

void InitStacks(Stack<char>[] stacksArr, string[] initialStateLines) {
    foreach (var initialStateLine in initialStateLines.Reverse().Skip(1)) {
        Regex pattern = CratePattern();
        var matches = pattern.Matches(initialStateLine);
        for (var i = 0; i < matches.Count; i++) {
            var match = matches[i];
            var group = match.Groups[1];
            if (string.IsNullOrWhiteSpace(group.Value))
                continue;
            var crateId = group.Value[1];
            stacks[i].Push(crateId);
        }
    }
}


var instructionLines = instructionsString.Split(Environment.NewLine);
var instructions = instructionLines.Select(EncodeInstruction);

void PartOne(Stack<char>[] stacks, IEnumerable<(int count, int from, int to)> instructions) {
    foreach (var (count, from, to) in instructions) {
        for (int i = 0; i < count; i++) {
            stacks[to].Push(stacks[from].Pop());
        }
    }
}

InitStacks(stacks, initialStateLines);
PartOne(stacks, instructions);

var topCrates = stacks.Select(stack => stack.Peek());

Console.WriteLine(new string(topCrates.ToArray()));


// part two

void PartTwo(Stack<char>[] stacks, IEnumerable<(int count, int from, int to)> instructions) {
    foreach (var (count, from, to) in instructions) {
        var holder = new char[count];
        for (int i = 0; i < count; i++) {
            holder[i] = stacks[from].Pop();
        }
        foreach (var item in holder.Reverse())
            stacks[to].Push(item);
    }
}

for (var i = 0; i < numStacks; i++)
    stacks[i] = new Stack<char>();

InitStacks(stacks, initialStateLines);
PartTwo(stacks, instructions);


topCrates = stacks.Select(stack => stack.Peek());

Console.WriteLine(new string(topCrates.ToArray()));

















partial class Program {
    [GeneratedRegex("move (?<count>\\d+) from (?<from>\\d+) to (?<to>\\d+)")]
    private static partial Regex InstructionLinePattern();
}

partial class Program {
    [GeneratedRegex(@"(\[[A-Z]\]|   ) ?")]
    private static partial Regex CratePattern();
}