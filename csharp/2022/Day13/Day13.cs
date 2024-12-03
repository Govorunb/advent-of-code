global using AoC2022.Common;
using Day13;
using System.Diagnostics;

string testInput = """
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
""";

int p1TestOutput = 13;

string input = File.ReadAllText("Input.txt");

// common function defs
(_List First, _List Second)[] ProcessInput(string input) {
    var strings = input.Split(StringExtensions.DoubleNewLine);
    var lines = strings.Select(pair => pair.Lines());
    var lists = lines.Select(lineArr => lineArr.Select(_List.FromText).ToArray());
    var pairs = lists.Select(listArr => (listArr[0], listArr[1]));

    return pairs.ToArray();
}

// part one
int PartOne(string input) {
    var packetPairs = ProcessInput(input);
    var correctPairs = packetPairs.WithIndex()
        .Where(pair => pair.Element.First
            .CompareTo(pair.Element.Second) < 1);
    return correctPairs.Sum(pair => pair.Index + 1);

}

Debug.Assert(PartOne(testInput) == p1TestOutput);

Console.WriteLine(PartOne(input));

// part two

int p2TestOutput = 140;

int PartTwo(string input) {
    var packetPairs = ProcessInput(input);
    var flattened = packetPairs.SelectMany(pair => new[] { pair.First, pair.Second });
    var firstDivider = _List.FromText("[[2]]");
    var secondDivider = _List.FromText("[[6]]");

    var withDividers = flattened.Append(firstDivider).Append(secondDivider);

    var ordered = withDividers.Order().ToList();
    return (ordered.IndexOf(firstDivider) + 1) * (ordered.IndexOf(secondDivider) + 1);
}

Debug.Assert(PartTwo(testInput) == p2TestOutput);

Console.WriteLine(PartTwo(input));
