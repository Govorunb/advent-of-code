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

input = File.ReadAllText("Input.txt");


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