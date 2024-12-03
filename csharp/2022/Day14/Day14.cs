global using AoC2022.Common;
using System.Diagnostics;
using System.Diagnostics.CodeAnalysis;
using System.Drawing;
using System.Text;

string testInput = """
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
""";

int p1TestOutput = 24;

string input = File.ReadAllText("Input.txt");

// common function defs


// part one
int PartOne(string input) {
    var grid = new SandGrid(input);
    var sandOrigin = new Point(500, 0);
    grid[sandOrigin] = Cell.SandSource;
    Console.WriteLine(grid);
    Console.WriteLine('\n');
    int count = 0;
    while (true) {
        var path = grid.DropSand(sandOrigin);
        var last = path.LastOrDefault();
        if (last == Point.Empty || last.Y == int.MaxValue || last == sandOrigin) {
            foreach (var pt in path.Where(grid.Bounds.Contains))
                grid[pt] = Cell.EndlessFallPath;
            break;
        }
        grid[last] = Cell.SandSettled;
        count++;
    }
    Console.WriteLine(grid);
    Console.WriteLine('\n');
    Console.WriteLine(count);
    return count;
}

Debug.Assert(PartOne(testInput) == p1TestOutput);

if (input != "real input here")
    Console.WriteLine(PartOne(input));

// part two

int p2TestOutput = 93;

int PartTwo(string input) {
    var grid = new SandGrid(input, true);
    var sandOrigin = new Point(500, 0);
    grid[sandOrigin] = Cell.SandSource;
    Console.WriteLine(grid);
    Console.WriteLine('\n');
    int count = 0;
    while (true) {
        var path = grid.DropSand(sandOrigin);
        var last = path.LastOrDefault(sandOrigin);
        grid[last] = Cell.SandSettled;
        count++;
        if (last == sandOrigin) {
            break;
        }
    }
    Console.WriteLine(grid);
    Console.WriteLine('\n');
    Console.WriteLine(count);
    return count;
}

Debug.Assert(PartTwo(testInput) == p2TestOutput);

if (input != "real input here")
    Console.WriteLine(PartTwo(input));


enum Cell : ushort {
    Empty = '.',
    Rock = '#',
    SandSource = '+',
    SandFalling = '*',
    SandSettled = 'o',
    EndlessFallPath = '~',
}

class SandGrid {
    public Point TopLeft; // start
    public Point BottomRight; // end
    public Rectangle Bounds => new(TopLeft, new Size(Width, Height));
    private Rectangle LocalBounds => new(ToLocal(TopLeft), new Size(Width, Height));

    public Cell[,] _grid;

    public int Height, Width;


    public Dictionary<int, int> Heightmap = new();

    public Cell this[Point pt] {
        get {
            var local = ToLocal(pt);
            return _grid[local.Y, local.X];
        }
        set {
            var local = ToLocal(pt);
            _grid[local.Y, local.X] = value;
        }
    }

    public Cell this[int row, int col] {
        get => _grid[row, col];
        set => _grid[row, col] = value;
    }

    public SandGrid(string input, bool partTwo = false) {
        var lines = input.Lines();
        var points = lines.Select(line =>
            line.Split(" -> ")
                .Select(ptStr => ptStr.Split(',').Select(int.Parse).ToArray())
                .Select(ptArr => new Point(ptArr[0], ptArr[1]))
        );
        Init(points, partTwo);
    }

    [MemberNotNull(nameof(_grid))]
    public void Init(IEnumerable<IEnumerable<Point>> wallVertices, bool partTwo = false) {
        // (500,0) must always be in view
        int minX = 500;
        int minY = 0;
        int maxX = 500;
        int maxY = 0;
        foreach (var pt in wallVertices.Flatten()) {
            minX = Math.Min(minX, pt.X);
            minY = Math.Min(minY, pt.Y);
            maxX = Math.Max(maxX, pt.X);
            maxY = Math.Max(maxY, pt.Y);
        }

        TopLeft = new Point(minX, minY);
        BottomRight = new Point(maxX, maxY);
        if (partTwo) {
            maxY += 2;
            // end pattern is an isosceles triangle
            // so the most width we'll possibly need is based on the height (exactly equal to it in fact)
            // (500,0) may not be equidistant from the sides so let's fix that
            TopLeft.X = int.Min(TopLeft.X, 500-maxY);
            BottomRight.X = int.Max(BottomRight.X, 500+maxY);
            BottomRight.Y += 2;
        }
        var size = new Size(BottomRight.OffsetFrom(TopLeft));
        (Width, Height) = (size.Width+1, size.Height+1);

        _grid = new Cell[Height, Width];
        for (var i = 0; i < Height; i++) {
            for (var j = 0; j < Width; j++) {
                _grid[i, j] = Cell.Empty;
            }
        }

        foreach (var wall in wallVertices) {
            Point lineStart;
            Point lineEnd = wall.First();
            foreach (var point in wall) {
                if (lineEnd == point)
                    continue;
                lineStart = lineEnd;
                lineEnd = point;
                DrawRockLine(lineStart, lineEnd, coordsAreGlobal: true);
            }
        }

        if (partTwo) {
            // floor
            DrawRockLine(new Point(TopLeft.X, BottomRight.Y), BottomRight, coordsAreGlobal: true);
        }
    }

    public void DrawRockLine(Point start, Point end, bool coordsAreGlobal) {
        IEnumerable<Point> linePoints = start.X == end.X ? // vertical
            Enumerable.Range(int.Min(start.Y, end.Y), int.Abs(start.Y - end.Y)+1)
                .Select(y => new Point(start.X, y))
            : start.Y == end.Y ? // horizontal
                Enumerable.Range(int.Min(start.X, end.X), int.Abs(start.X - end.X)+1)
                    .Select(x => new Point(x, start.Y))
                    : throw new ArgumentException("only straight lines");
        var highestPointInLine = int.Min(start.Y, end.Y);
        foreach (var pt in linePoints) {
            var (local, global) = coordsAreGlobal ? (ToLocal(pt), pt) : (pt, ToGlobal(pt));
            // don't use local from this point on, only global or local (to disambiguate)
            Heightmap[global.X] = Heightmap[local.X] = int.Min(Heightmap.GetValueOrDefault(local.X, int.MaxValue), highestPointInLine);
            _grid[local.Y, local.X] = Cell.Rock;
        }
    }

    public Point ToLocal(Point global) {
        return global.OffsetFrom(TopLeft);
    }

    public Point ToGlobal(Point local) {
        if (local.Y == int.MaxValue)
            return new Point(TopLeft.X + local.X, int.MaxValue);
        return TopLeft + new Size(local);
    }

    /// <param name="origin">Location of the sand spawner.</param>
    /// <param name="path">Path the sand grain took towards its destination.<br/>If the grain fell off the edge, the last point will have the X coordinate of where it fell and a Y coordinate of <see cref="int.MaxValue"/>.</param>
    /// <returns>A <see langword="bool"/> indicating whether the grain successfully settled (<see langword="true"/>) or fell off the edge (<see langword="false"/>).</returns>
    public IEnumerable<Point> DropSand(Point? origin = null) {
        var curr = ToLocal(origin ?? new Point(500, 0));
        while (true) {
            var nextStep = SandStep(curr);
            if (nextStep.Y == int.MaxValue) {
                yield return ToGlobal(nextStep);
                yield break;
            }
            //var prev = _grid[nextStep.Y, nextStep.X];
            //_grid[nextStep.Y, nextStep.X] = Cell.SandFalling;
            //Console.WriteLine(this);
            //_grid[nextStep.Y, nextStep.X] = prev;
            if (nextStep == curr) {
                // settled
                Heightmap[ToGlobal(nextStep).X] = Heightmap[nextStep.X] = int.Min(Heightmap.GetValueOrDefault(nextStep.X, int.MaxValue), nextStep.Y);
                yield break;
            }
            yield return ToGlobal(nextStep);
            curr = nextStep;
        }
        
    }

    public Cell Below(Point pt)
        => Below(pt.X, pt.Y);
    public Cell Below(int x, int y)
        => CellAtOffset(x, y, 0, 1);

    public Cell BottomLeftOf(Point pt)
        => BottomLeftOf(pt.X, pt.Y);
    public Cell BottomLeftOf(int x, int y)
        => CellAtOffset(x, y, -1, 1);

    public Cell BottomRightOf(Point pt)
        => BottomRightOf(pt.X, pt.Y);
    public Cell BottomRightOf(int x, int y)
        => CellAtOffset(x, y, 1, 1);

    private Cell CellAtOffset(int x, int y, int xOffset, int yOffset) {
        var (targetX, targetY) = (x + xOffset, y + yOffset);
        return LocalBounds.Contains(targetX, targetY) ? _grid[targetY, targetX] : Cell.Empty;
    }

    public Point SandStep(Point grain) {
        var local = grain.X > 400 ? ToLocal(grain) : grain;
        Point res = local;

        if (Below(local) == Cell.Empty) {
            // "drop to ground" without waiting several steps
            if (!Heightmap.TryGetValue(res.X, out int storedTop)) // no platform, falls off
                return new Point(res.X, int.MaxValue);
            else {
                if (storedTop > res.Y)
                    res.Y = storedTop-1;
                else {
                    // below the top shelf, perform slow fall
                    res.Offset(0, 1);
                }
            }
        }
        else if (BottomLeftOf(local) == Cell.Empty) {
            res.Offset(-1, 1);
        }
        else if (BottomRightOf(local) == Cell.Empty) {
            res.Offset(1, 1);
        }

        if (!LocalBounds.Contains(res)) // L + you fell off
            res.Y = int.MaxValue;

        return res;
    }

    public override string ToString() {
        var repr = new StringBuilder();
        for (var i = -3; i < Height; i++) {
            if (i >= 0)
                repr.Append($"{i,3} ");
            else
                repr.Append($"    ");
            for (var j = 0; j < Width; j++) {
                if (i < 0)
                    repr.Append((TopLeft.X + j).ToString()[3+i]);
                else
                    repr.Append((char)_grid[i, j]);
            }
            repr.AppendLine();
        }
        return repr.ToString();
    }
}