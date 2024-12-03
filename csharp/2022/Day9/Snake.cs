using System.Drawing;

namespace Day9; 
internal record Snake {
    public Point[] Points;

    public Snake(int segmentCount) {
        Points = new Point[segmentCount];
    }

    public Rectangle RelaxedRect(Point segment)
        => new(segment.X - 1, segment.Y - 1, 3, 3);

    public bool Relaxed(int i) {
        if (i == 0) return true;
        var tail = Points[i];
        var head = Points[i - 1];
        return RelaxedRect(tail).Contains(head);
    }

    public IEnumerable<Point> Move(Direction direction, int amt) {
        return Enumerable.Range(0, amt)
            .Select(_ => StepWhole(direction))
            .Distinct();
    }

    public Point StepWhole(Direction direction) {
        for (var i = 0; i < Points.Length; i++) {
            if (!UpdateSegment(i, direction))
                break;
        }

        return Points[^1];
    }

    public bool UpdateSegment(int i, Direction direction) {
        if (i == 0) {
            var snakeHead = Points[i];
            switch (direction) {
                case Direction.Left:
                    snakeHead.X--;
                    break;
                case Direction.Right:
                    snakeHead.X++;
                    break;
                case Direction.Up:
                    snakeHead.Y++;
                    break;
                case Direction.Down:
                    snakeHead.Y--;
                    break;
            }
            Points[i] = snakeHead;
            return true;
        }
        if (Relaxed(i))
            return false;
        var tail = Points[i];
        var head = Points[i - 1];
        var diff = head.OffsetFrom(tail);
        if (diff.Y != 0) {
            tail.Y += diff.Y < 0 ? -1 : 1;
        }
        if (diff.X != 0) {
            tail.X += diff.X < 0 ? -1 : 1;
        }
        Points[i] = tail;
        return true;
    }
}
