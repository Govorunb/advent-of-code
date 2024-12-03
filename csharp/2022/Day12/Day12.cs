global using AoC2022.Common;
using Day12;
using System.Diagnostics;
using System.Text;

string testInput = """
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
""";

int p1TestOutput = 31;

string input = File.ReadAllText("Input.txt");

// common function defs

// part one
int PartOne(string input) {
    var grid = new HeightmapGrid(input);
    var sb = new StringBuilder();
    for (var y = 0; y < grid.Height; y++) {
        for (var x = 0; x < grid.Width; x++) {
            var cell = grid[y, x];
            sb.Append(cell.Repr);
        }
        sb.AppendLine();
    }
    var gridRepr = sb.ToString();
    Console.WriteLine(gridRepr);
    var visitedCells = grid.Width * grid.Height - gridRepr.CountOf('.') - 1;
    return visitedCells;
}

Debug.Assert(PartOne(testInput) == p1TestOutput);

Console.WriteLine(PartOne(input));

// part two

int p2TestOutput = 29;

int PartTwo(string input) {
    var grid = new HeightmapGrid(input, true);
    var sb = new StringBuilder();
    for (var y = 0; y < grid.Height; y++) {
        for (var x = 0; x < grid.Width; x++) {
            var cell = grid[y, x];
            sb.Append(cell.Repr);
        }
        sb.AppendLine();
    }
    var gridRepr = sb.ToString();
    Console.WriteLine(gridRepr);
    var visitedCells = grid.Width * grid.Height - gridRepr.CountOf('.') - 1;
    return visitedCells;
}

Debug.Assert(PartTwo(testInput) == p2TestOutput);

Console.WriteLine(PartTwo(input));
