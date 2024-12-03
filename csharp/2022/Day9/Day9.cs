global using AoC2022.Common;
using Day9;
using System.Diagnostics;
using System.Drawing;

string p1TestInput = """
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
""";

int p1TestOutput = 13;

string input = File.ReadAllText("Input.txt");

// common function defs
Dictionary<char, Direction> DirectionMap = new()
{
    { 'U', Direction.Up },
    { 'D', Direction.Down },
    { 'L', Direction.Left },
    { 'R', Direction.Right },
};

// part one
int PartOne(string input) {

    var lines = input.Split(Environment.NewLine);
    var instructions = lines.Select<string, (Direction direction, int amt)>(line => (DirectionMap[line[0]], int.Parse(line[2..])));

    HashSet<Point> visited = new() { new Point(0, 0) };
    Snake snake = new(2);
    foreach (var (direction, amt) in instructions) {
        visited.AddRange(snake.Move(direction, amt));
    }

    return visited.Count;
}

Debug.Assert(PartOne(p1TestInput) == p1TestOutput);

Console.WriteLine(PartOne(input));

// part two
string p2TestInput = """
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
""";

int p2TestOutput = 36;

int PartTwo(string input) {

    var lines = input.Split(Environment.NewLine);
    var instructions = lines.Select<string, (Direction direction, int amt)>(line => (DirectionMap[line[0]], int.Parse(line[2..])));

    HashSet<Point> visited = new() { new Point(0, 0) };
    Snake snake = new(10);
    foreach (var (direction, amt) in instructions) {
        visited.AddRange(snake.Move(direction, amt));
    }

    return visited.Count;
}

Debug.Assert(PartTwo(p2TestInput) == p2TestOutput);


Console.WriteLine(PartTwo(input));
