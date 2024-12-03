global using AoC2022.Common;
using System.Collections;
using System.Collections.Generic;
using System.Diagnostics;
using System.Drawing;
using System.Text.RegularExpressions;

string testInput = """
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
""";

int p1TestOutput = 26;

string input = """
Sensor at x=2150774, y=3136587: closest beacon is at x=2561642, y=2914773
Sensor at x=3983829, y=2469869: closest beacon is at x=3665790, y=2180751
Sensor at x=2237598, y=3361: closest beacon is at x=1780972, y=230594
Sensor at x=1872170, y=78941: closest beacon is at x=1780972, y=230594
Sensor at x=3444410, y=3965835: closest beacon is at x=3516124, y=3802509
Sensor at x=3231566, y=690357: closest beacon is at x=2765025, y=1851710
Sensor at x=3277640, y=2292194: closest beacon is at x=3665790, y=2180751
Sensor at x=135769, y=50772: closest beacon is at x=1780972, y=230594
Sensor at x=29576, y=1865177: closest beacon is at x=255250, y=2000000
Sensor at x=3567617, y=3020368: closest beacon is at x=3516124, y=3802509
Sensor at x=1774477, y=148095: closest beacon is at x=1780972, y=230594
Sensor at x=1807041, y=359900: closest beacon is at x=1780972, y=230594
Sensor at x=1699781, y=420687: closest beacon is at x=1780972, y=230594
Sensor at x=2867703, y=3669544: closest beacon is at x=3516124, y=3802509
Sensor at x=1448060, y=201395: closest beacon is at x=1780972, y=230594
Sensor at x=3692914, y=3987880: closest beacon is at x=3516124, y=3802509
Sensor at x=3536880, y=3916422: closest beacon is at x=3516124, y=3802509
Sensor at x=2348489, y=2489095: closest beacon is at x=2561642, y=2914773
Sensor at x=990761, y=2771300: closest beacon is at x=255250, y=2000000
Sensor at x=1608040, y=280476: closest beacon is at x=1780972, y=230594
Sensor at x=2206669, y=1386195: closest beacon is at x=2765025, y=1851710
Sensor at x=3932320, y=3765626: closest beacon is at x=3516124, y=3802509
Sensor at x=777553, y=1030378: closest beacon is at x=255250, y=2000000
Sensor at x=1844904, y=279512: closest beacon is at x=1780972, y=230594
Sensor at x=2003315, y=204713: closest beacon is at x=1780972, y=230594
Sensor at x=2858315, y=2327227: closest beacon is at x=2765025, y=1851710
Sensor at x=3924483, y=1797070: closest beacon is at x=3665790, y=2180751
Sensor at x=1572227, y=3984898: closest beacon is at x=1566446, y=4774401
Sensor at x=1511706, y=1797308: closest beacon is at x=2765025, y=1851710
Sensor at x=79663, y=2162372: closest beacon is at x=255250, y=2000000
Sensor at x=3791701, y=2077777: closest beacon is at x=3665790, y=2180751
Sensor at x=2172093, y=3779847: closest beacon is at x=2561642, y=2914773
Sensor at x=2950352, y=2883992: closest beacon is at x=2561642, y=2914773
Sensor at x=3629602, y=3854760: closest beacon is at x=3516124, y=3802509
Sensor at x=474030, y=3469506: closest beacon is at x=-452614, y=3558516
""";

// common function defs
Sensor ProcessLine(string line) {
    var match = SensorLinePattern().Match(line);
    Point sensorLoc = new(int.Parse(match.Groups["sensorX"].Value), int.Parse(match.Groups["sensorY"].Value));
    Point beaconLoc = new(int.Parse(match.Groups["beaconX"].Value), int.Parse(match.Groups["beaconY"].Value));
    return new(sensorLoc, beaconLoc);
}

// part one
int PartOne(string input, Line targetLine) {
    var lines = input.Lines();
    
    var sensors = lines.Select(ProcessLine).ToArray();

    IEnumerable<Point> nonBeaconPts = sensors.SelectMany(sensor => sensor.PointsOnLine(targetLine))
        .Distinct()
        .Except(sensors.Select(sensor => sensor.Beacon))
        .OrderBy(pt => targetLine.Orientation == LineOrientation.Horizontal ? pt.X : pt.Y);

    return nonBeaconPts.Count();
}

Debug.Assert(PartOne(testInput, (LineOrientation.Horizontal, 10)) == p1TestOutput);

if (false && input != "real input here")
    Console.WriteLine(PartOne(input, (LineOrientation.Horizontal, 2000000)));

// part two

long TuningFrequency(Point pt) {
    return (pt.X) * 4000000L + pt.Y;
}

int p2TestOutput = 56000011;

long PartTwo(string input, int maxCoord) {
    var lines = input.Lines();

    var sensors = lines.Select(ProcessLine).ToArray();
    Point beacon = Point.Empty;
    Rectangle bounds = new(0,0,maxCoord+1,maxCoord+1);
    // bruteforce didn't work
    //Parallel.For(0, maxCoord + 1, (int y, ParallelLoopState loopState) => {
    //    Line line = (LineOrientation.Horizontal, y);
    //    var coveredPoints = sensors.SelectMany(sensor => sensor.PointsOnLine(line))
    //        .Where(pt => pt.X >= 0 && pt.X <= maxCoord && pt.Y >= 0 && pt.Y <= maxCoord)
    //        .Select(pt => pt.X)
    //        .Distinct();
    //    var numCovered = coveredPoints.Count();
    //    if (numCovered >= maxCoord+1) {
    //        coveredPoints = null;
    //        GC.Collect();
    //        return;
    //    }
    //    for (var x = 0; x <= maxCoord; x++) {
    //        if (loopState.ShouldExitCurrentIteration)
    //            break;
    //        var pt = new Point(x, y);
    //        if (!sensors.Any(sensor => sensor.IsCovered(pt))) {
    //            beacon = pt;
    //            loopState.Break();
    //            break;
    //        }
    //    }
    //});

    // let's try narrowing down the search space a little bit
    // theoretically, if there's only one position suitable for the beacon,
    // it must be surrounded by "covered" points
    // which means it is a distance of 1 away from a few sensors' edges
    // now instead of (4e6+1)^2 it's only (~3e6)*4*(~50) :)
    
    foreach (var sensor in sensors) {
        foreach (Side vSide in new[] { Side.Top, Side.Bottom }) {
            foreach (Side hSide in new[] { Side.Left, Side.Right }) {
                for (var xOffset = 0; xOffset <= sensor.Radius; xOffset++) {
                    var yOffset = sensor.Radius - xOffset;
                    var offset = new Size(hSide == Side.Left ? -xOffset : xOffset, vSide == Side.Top ? -yOffset : yOffset);
                    var edgePoint = sensor.Location + offset;
                    Corner edgeSide = (vSide, hSide) switch
                    {
                        (Side.Top, Side.Left) => Corner.TopLeft,
                        (Side.Top, Side.Right) => Corner.TopRight,
                        (Side.Bottom, Side.Left) => Corner.BottomLeft,
                        (Side.Bottom, Side.Right) => Corner.BottomRight,
                        _ => throw new NotImplementedException(),
                    };
                    var offEdgePoint = edgePoint + edgeSide switch
                    {
                        // clockwise
                        Corner.TopLeft => new Size(-1, 0), // left
                        Corner.TopRight => new Size(0, -1), // top
                        Corner.BottomRight => new Size(1, 0), // right
                        Corner.BottomLeft => new Size(0, 1), // bottom
                        _ => throw new NotImplementedException(),
                    };
                    if (!bounds.Contains(offEdgePoint) || sensors.Any(sensor => sensor.IsCovered(offEdgePoint)))
                        continue;
                    else {
                        beacon = offEdgePoint;
                        return TuningFrequency(beacon);
                    }
                }
            }
        }
    }

    return TuningFrequency(beacon);
}

Debug.Assert(PartTwo(testInput, 20) == p2TestOutput);

if (input != "real input here")
    Console.WriteLine(PartTwo(input, 4000000));

enum LineOrientation {
    /// <summary>-Inf &lt; X &lt; Inf, fixed Y</summary>
    Horizontal,
    /// <summary>Fixed X, -Inf &lt; Y &lt; Inf</summary>
    Vertical
}

[DebuggerDisplay("Line({Orientation},{FixedAxisCoordinate})")]
struct Line {
    public LineOrientation Orientation;
    public int FixedAxisCoordinate;

    public static implicit operator Line((LineOrientation o, int c) tuple)
        => new() { Orientation = tuple.o, FixedAxisCoordinate = tuple.c };
}

class Sensor {

    public Point Location;
    public Point Beacon;
    public int Radius;

    public Sensor(Point location, Point beacon) {
        Location = location;
        Beacon = beacon;
        var radiusLine = Beacon.OffsetFrom(location);
        Radius = int.Abs(radiusLine.X) + int.Abs(radiusLine.Y);
    }

    public IReadOnlyCollection<Point> PointsOnLine(Line line) {
        return new LineIntersectionPointCollection(this, line);
    }
    public bool IsCovered(Point pt) {
        var offset = pt.OffsetFrom(Location);
        var dist = int.Abs(offset.X) + int.Abs(offset.Y);
        return dist <= Radius;
    }

    public IEnumerable<Point> CoveredPoints() {
        var minY = Location.Y - Radius;
        var maxY = Location.Y + Radius;
        return Enumerable.Range(minY, Radius * 2 + 1)
            .SelectMany(y => PointsOnLine((LineOrientation.Horizontal, y)));
    }

    private class LineIntersectionPointCollection : IReadOnlyCollection<Point> {
        Sensor Sensor;
        Line Line;

        Side Side;
        int Dist;
        public LineIntersectionPointCollection(Sensor sensor, Line line) {
            Sensor = sensor;
            Line = line;
            // determine if the sensor's range contains any part of the line
            // this actually tests the square around the range, but we're not looking for a single point so
            var fixedAxisAnchor = line.Orientation switch
            {
                LineOrientation.Horizontal => Sensor.Location.Y,
                LineOrientation.Vertical => Sensor.Location.X,
                _ => throw new ArgumentException($"Invalid {nameof(LineOrientation)}", nameof(line))
            };
            int distInwardsFromMin = line.FixedAxisCoordinate - (fixedAxisAnchor - Sensor.Radius);
            int distInwardsFromMax = (fixedAxisAnchor + Sensor.Radius) - line.FixedAxisCoordinate;
            bool crossesLine = distInwardsFromMin >= 0 && distInwardsFromMax >= 0;
            if (!crossesLine)
                return;

            // sensor area is a diamond similar to Day 14's part two
            // so wherever the line of intersecting points is in relation to the sensor (provided it's within the radius)
            // it has to be (1) as wide as it is tall; and (2) centered on the non-fixed axis coordinate ("other" coordinate) of the sensor's location

            // N = 1+2|v-l|, where N is the number of intersecting points, v is the vertex of the diamond closest to the line, and l is the line

            // pick side closest to the line (least distance from it)
            Side = line.Orientation switch
            {
                LineOrientation.Horizontal => distInwardsFromMax > distInwardsFromMin ? Side.Top : Side.Bottom, // is up higher Y or lower?
                LineOrientation.Vertical => distInwardsFromMax > distInwardsFromMin ? Side.Left : Side.Right,   // technically we can be wrong as long as we're entirely consistent
                _ => throw new NotImplementedException(),
            };
            Dist = int.Min(distInwardsFromMin, distInwardsFromMax);
            _count = 1 + 2 * Dist;
        }

        private int _count;
        public int Count => _count;

        public IEnumerator<Point> GetEnumerator()
            => GetPoints().GetEnumerator();

        IEnumerator IEnumerable.GetEnumerator()
            => GetEnumerator();

        private IEnumerable<Point> GetPoints() {
            if (_count == 0)
                yield break;
            // at dist 0 there's 1 square
            Point center = Line.Orientation switch
            {
                LineOrientation.Horizontal => new(Sensor.Location.X, Line.FixedAxisCoordinate),
                LineOrientation.Vertical => new(Line.FixedAxisCoordinate, Sensor.Location.Y),
                _ => throw new NotImplementedException(),
            };
            yield return center;
            // at dist 1 there's 3, at 2 there's 5, etc.
            for (var i = 1; i <= Dist; i++) {
                int offset = Side switch
                {
                    Side.Left or Side.Top => -i,
                    Side.Right or Side.Bottom => i,
                    _ => throw new NotImplementedException(),
                };
                // first one side
                yield return Line.Orientation switch
                {
                    LineOrientation.Horizontal => new(center.X + offset, Line.FixedAxisCoordinate),
                    LineOrientation.Vertical => new(Line.FixedAxisCoordinate, center.Y + offset),
                    _ => throw new NotImplementedException(),
                };
                // and then the other
                yield return Line.Orientation switch
                {
                    LineOrientation.Horizontal => new(center.X - offset, Line.FixedAxisCoordinate),
                    LineOrientation.Vertical => new(Line.FixedAxisCoordinate, center.Y - offset),
                    _ => throw new NotImplementedException(),
                };
            }
        }
    }
}

partial class Program {
    [GeneratedRegex("Sensor at x=(?<sensorX>-?\\d+), y=(?<sensorY>-?\\d+): closest beacon is at x=(?<beaconX>-?\\d+), y=(?<beaconY>-?\\d+)")]
    private static partial Regex SensorLinePattern();
}