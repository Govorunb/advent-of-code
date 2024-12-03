using QuickGraph;
using QuickGraph.Algorithms;
using System.Drawing;

#pragma warning disable CS8618 // Non-nullable field must contain a non-null value when exiting constructor. Consider declaring as nullable.
namespace Day12; 
internal class HeightmapGrid {
    public readonly int StartElevation = ElevationFromChar('a');
    public readonly int EndElevation = ElevationFromChar('z');

    public static int ElevationFromChar(char c)
        => c - 'a';

    public GridCell Start;
    public GridCell End;

    public int Height, Width;
    private GridCell[,] _cells;

    public GridCell this[int row, int col] {
        get => _cells[row, col];
    }

    public GridCell this[Point pt] {
        get => _cells[pt.Y, pt.X];
    }

    public HeightmapGrid(string input, bool partTwo = false) {
        BidirectionalGraph<GridCell, HeightmapEdge<GridCell>> graph = new(false);
        var lines = input.Lines();
        Height = lines.Length;
        Width = lines[0].Length;
        _cells = new GridCell[Height, Width];
        foreach (var (line, y) in lines.WithIndex()) {
            foreach (var (letter, x) in line.WithIndex()) {
                int height = letter switch
                {
                    'S' => StartElevation,
                    'E' => EndElevation,
                    _ => ElevationFromChar(letter)
                };
                var cell = new GridCell { Grid = this, Location = new Point(x, y), Elevation = height };
                if (letter is 'S')
                    Start = cell;
                else if (letter is 'E')
                    End = cell;
                _cells[y, x] = cell;
                graph.AddVertex(cell);
            }
        }
        if (Start is null || End is null)
            throw new Exception(); // shut up compiler

        (Size offset, Direction direction)[] neighbors = new[]
        {
            (new Size(0, -1), Direction.Up),
            (new Size(0, 1), Direction.Down),
            (new Size(1, 0), Direction.Right),
            (new Size(-1, 0), Direction.Left),
        };
        Rectangle gridBounds = new(0, 0, Width, Height);
        foreach (var cell in graph.Vertices) {
            foreach (var (offset, direction) in neighbors) {
                var neighborLoc = cell.Location + offset;
                if (gridBounds.Contains(neighborLoc)) {
                    var neighbor = _cells[neighborLoc.Y, neighborLoc.X];
                    if (neighbor.Elevation <= cell.Elevation + 1)
                        graph.AddEdge(new HeightmapEdge<GridCell>(cell, neighbor, direction));
                }
            }
        }
        var alg = new QuickGraph.Algorithms.ShortestPath.DijkstraShortestPathAlgorithm<GridCell, HeightmapEdge<GridCell>>
            (graph, (edge) => edge.Weight);
        var currentMin = int.MaxValue;
        var shortestPath = Enumerable.Empty<HeightmapEdge<GridCell>>();

        var searchSpace = !partTwo ? new[] { Start }.AsEnumerable()
            : _cells.Cast<GridCell>().Where(cell => cell.Elevation == 0).OrderBy(cell => {
                var vector = End.Location.OffsetFrom(cell.Location);
                return Math.Pow(vector.X, 2) * Math.Pow(vector.Y, 2);
            });

        foreach (var cell in searchSpace) {
            var tryFunc = graph.ShortestPathsAStar((_) => 1, (cell) => cell.Elevation, cell);
            if (tryFunc(End, out var path)) {
                int pathLen = path.Count();
                if (pathLen < currentMin) {
                    currentMin = pathLen;
                    shortestPath = path;
                }
            }
        }
        foreach (var edge in shortestPath) {
            edge.Source.ShortestPathDirection = edge.Direction;
        }
    }

    internal record GridCell {
        public HeightmapGrid Grid { get; init; }
        public int Elevation { get; init; }
        public Point Location { get; init; }
        public int X => Location.X;
        public int Y => Location.Y;

        public Direction? ShortestPathDirection = null;
        public char Repr => this == Grid.End ? 'E'
            : ShortestPathDirection switch
            {
                null => '.',
                Direction.Left => '<',
                Direction.Right => '>',
                Direction.Up => '^',
                Direction.Down => 'v',
                _ => '?',
            };
    }

}
public interface IWeightedEdge<TVertex> : IEdge<TVertex> {
    public long Weight { get; }
}

internal class HeightmapEdge<TVertex> : Edge<TVertex>, IWeightedEdge<TVertex> {
    public Direction Direction;
    public long Weight => 1;
    public HeightmapEdge(TVertex from, TVertex to, Direction direction) : base(from, to) {
        Direction = direction;
    }
}
