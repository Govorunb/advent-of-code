using System.Collections;

namespace Day8; 
internal enum Side { Left, Right, Top, Bottom }
internal class StaticGrid : IEnumerable<StaticGrid.GridCell> {
    private int[,] _grid;
    public int Width { get; init; }
    public int Height { get; init; }
    public StaticGrid(int width, int height) {
        Width = width;
        Height = height;
        _grid = new int[height, width];
    }

    public IEnumerable<int> Column(int columnIndex) {
        for (var rowIndex = 0; rowIndex < Height; rowIndex++) {
            yield return _grid[rowIndex, columnIndex];
        }
    }
    public IEnumerable<int> Row(int rowIndex) {
        for (var columnIndex = 0; columnIndex < Width; columnIndex++) {
            yield return _grid[rowIndex, columnIndex];
        }
    }

    private System.Data.DataTable? _vis;
    public System.Data.DataTable VisibilityMatrix {
        get {
            if (_vis is not null) return _vis;
            var table = new System.Data.DataTable();
            for (var i = 0; i < Width; i++)
                table.Columns.Add(i.ToString(), typeof(bool));
            for (var i = 0; i < Height; i++)
                table.Rows.Add();
            table.BeginLoadData();
            foreach (var row in Enumerable.Range(0, Height)) {
                foreach (var col in Enumerable.Range(0, Width)) {
                    table.Rows[row][col] = IsVisible(row, col);
                }
            }
            table.EndLoadData();
            return _vis = table;
        }
    }

    private System.Data.DataTable? _table;
    public System.Data.DataTable AsTable {
        get {
            if (_table is not null) return _table;
            var table = new System.Data.DataTable();
            for (var i = 0; i < Width; i++)
                table.Columns.Add(i.ToString(), typeof(int));
            for (var i = 0; i < Height; i++)
                table.Rows.Add();
            table.BeginLoadData();
            foreach (var row in Enumerable.Range(0, Height)) {
                foreach (var col in Enumerable.Range(0, Width)) {
                    table.Rows[row][col] = this[row, col].Value;
                }
            }
            table.EndLoadData();
            return _table = table;
        }
    }

    private System.Data.DataTable? _scores;
    public System.Data.DataTable ScoreMatrix {
        get {
            if (_scores is not null) return _scores;
            var table = new System.Data.DataTable();
            for (var i = 0; i < Width; i++)
                table.Columns.Add(i.ToString(), typeof(int));
            for (var i = 0; i < Height; i++)
                table.Rows.Add();
            table.BeginLoadData();
            foreach (var row in Enumerable.Range(0, Height)) {
                foreach (var col in Enumerable.Range(0, Width)) {
                    table.Rows[row][col] = ScenicScore(this[row, col]);
                }
            }
            table.EndLoadData();
            return _scores = table;
        }
    }

    public bool IsVisible(int rowIndex, int columnIndex)
        => IsVisible(rowIndex, columnIndex, Side.Left)
            || IsVisible(rowIndex, columnIndex, Side.Top)
            || IsVisible(rowIndex, columnIndex, Side.Bottom)
            || IsVisible(rowIndex, columnIndex, Side.Right);

    public bool IsVisible(int rowIndex, int columnIndex, Side from) {
        var val = _grid[rowIndex, columnIndex];
        IEnumerable<int> view;
        int targetIndex;
        switch (from) {
            case Side.Left:
                view = Row(rowIndex);
                targetIndex = columnIndex;
                break;
            case Side.Right:
                view = Row(rowIndex);
                targetIndex = (Height - 1) - columnIndex;
                break;
            case Side.Top:
                view = Column(columnIndex);
                targetIndex = (Width - 1) - rowIndex;
                break;
            case Side.Bottom:
                view = Column(columnIndex);
                targetIndex = rowIndex;
                break;
            default:
                return true; // will never happen shut up compiler
        }
        if (from is Side.Right or Side.Top) {
            view = view.Reverse();
        }
        var withIndex = view.WithIndex();
        return withIndex.First(item => item.Element >= val).Index >= targetIndex;
    }

    public IEnumerable<int> GetVisible((int row, int col) from, Direction direction) {
        var val = _grid[from.row, from.col];
        var view = direction switch
        {
            Direction.Left => Row(from.row).SkipLast(Width - from.col).Reverse(),
            Direction.Right => Row(from.row).Skip(from.col + 1),
            Direction.Up => Column(from.col).SkipLast(Height - from.row).Reverse(),
            Direction.Down => Column(from.col).Skip(from.row + 1),
            _ => Enumerable.Empty<int>(),
        };
        // var maxVisible = -1;
        foreach (var tree in view) {
            //if (tree < maxVisible) // obscured by preceding tree
            //    continue;
            yield return tree;
            //maxVisible = tree; // never lower than before
            if (tree >= val) // stop at the first tree that is the same height or taller
                yield break;
        }
    }

    public static int ScenicScore(StaticGrid.GridCell cell) {
        if (cell.OnEdge)
            return 0; // one of the sides will see 0 trees
        var (Left, Up, Down, Right) = cell.GetVisible();
        var (LeftCount, UpCount, DownCount, RightCount) = (Left.Count(), Up.Count(), Down.Count(), Right.Count());
        return Left.Count() * Up.Count() * Down.Count() * Right.Count();
    }

    public IEnumerator<GridCell> GetEnumerator() {
        return new GridEnumerator(this);
    }

    IEnumerator IEnumerable.GetEnumerator()
        => GetEnumerator();

    public GridCell this[int rowIndex, int columnIndex] {
        get => new(this)
        {
            RowIndex = rowIndex,
            ColumnIndex = columnIndex
        };
    }

    internal record GridCell {
        public int RowIndex { get; init; }
        public int ColumnIndex { get; init; }
        public bool OnEdge
            => RowIndex == 0 || ColumnIndex == 0
            || RowIndex == (Grid.Width - 1) || ColumnIndex == (Grid.Height - 1);
        public int Value {
            get => Grid._grid[RowIndex, ColumnIndex];
            set => Grid._grid[RowIndex, ColumnIndex] = value;
        }
        public StaticGrid Grid;
        public bool IsVisible => OnEdge || Grid.IsVisible(RowIndex, ColumnIndex);

        public GridCell(StaticGrid grid) {
            Grid = grid;
        }

        public (IEnumerable<int> Left, IEnumerable<int> Up, IEnumerable<int> Down, IEnumerable<int> Right) GetVisible()
            => (GetVisible(Direction.Left), GetVisible(Direction.Up), GetVisible(Direction.Down), GetVisible(Direction.Right));
        public IEnumerable<int> GetVisible(Direction direction)
            => Grid.GetVisible((RowIndex, ColumnIndex), direction);
    }

    internal class GridEnumerator : IEnumerator<GridCell> {

        private readonly StaticGrid _grid;
        private int rowIdx = 0;
        private int colIdx = -1; // why did nobody tell me enumerators are supposed to start from nothing and get the first item after the first MoveNext

        public GridEnumerator(StaticGrid grid) {
            _grid = grid;
        }
        public GridCell Current => _grid[rowIdx, colIdx];

        object IEnumerator.Current => Current;

        public bool MoveNext() {
            colIdx++;
            if (colIdx >= _grid.Width) {
                colIdx = 0;
                rowIdx++;
            }
            if (rowIdx >= _grid.Height) {
                return false;
            }
            return true;
        }

        public void Reset() {
            rowIdx = 0;
            colIdx = -1;
        }

        private bool _disposed;
        protected virtual void Dispose(bool disposing) {
            if (!_disposed) {
                if (disposing) {
                    // dispose managed state (managed objects)
                }

                // free unmanaged resources (unmanaged objects) and override finalizer
                // set large fields to null
                _disposed = true;
            }
        }

        public void Dispose() {
            // Do not change this code. Put cleanup code in 'Dispose(bool disposing)' method
            Dispose(disposing: true);
            GC.SuppressFinalize(this);
        }
    }
}
