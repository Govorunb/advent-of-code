global using AoC2022.Common;
using Day8;
using System.Diagnostics;

List<string> testInputs = new()
{
    """
    30373
    25512
    65332
    33549
    35390
    """,
    """
    2494568
    6952220
    1715513
    7968924
    0485697
    0673086
    3684009
    """,
};

List<int> p1TestOutputs = new() {
    21,
    41
};

string input = File.ReadAllText("Input.txt");

// common function defs
StaticGrid Map(string input) {
    var lines = input.Split(Environment.NewLine);
    (int width, int height) = (lines[0].Length, lines.Length);
    var grid = new StaticGrid(width, height);
    foreach (var (line, rowIndex) in lines.WithIndex()) {
        foreach (var (c, columnIndex) in line.WithIndex()) {
            grid[rowIndex, columnIndex].Value = c - '0';
        }
    }
    _ = grid.VisibilityMatrix;
    _ = grid.ScoreMatrix;
    _ = grid.AsTable;
    return grid;
}

// part one
int PartOne(string input) {
    var grid = Map(input);
    var visible = grid.Count(x => x.IsVisible);

    return visible;
}

Debug.Assert(testInputs.Zip(p1TestOutputs).All(pair => PartOne(pair.First) == pair.Second));

Console.WriteLine(PartOne(input));

// part two

List<int> p2TestOutputs = new() {
    8,
    18
};




int PartTwo(string input) {
    var map = Map(input);
    //return map.Select(StaticGrid.ScenicScore).Max();
    var maxScore = 0;
    foreach (var cell in map) {
        var scenicScore = StaticGrid.ScenicScore(cell);

        maxScore = Math.Max(maxScore, scenicScore);
    }
    return maxScore;
}

Debug.Assert(testInputs.Zip(p2TestOutputs).All(pair => PartTwo(pair.First) == pair.Second));

Console.WriteLine(PartTwo(input));
