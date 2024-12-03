namespace Day7; 
internal class _File {
    public string Name { get; set; }
    public virtual int Size { get; private set; }

    private _Directory _parent = _Directory.Root;
    public virtual _Directory Parent {
        get => _parent;
        set {
            _parent?.Files.Remove(Name);
            _parent = value;
            _parent?.Files.Add(Name, this);
        }
    }
    public string Path => (Parent is null || Parent == this) ? Name : System.IO.Path.Join(Parent.Path, Name);
    public _File(string name, int size, _Directory parent) {
        Name = name;
        Size = size;
        Parent = parent;
    }
}

internal class _Directory : _File {
    public static _Directory Root = new("", null);
    public Dictionary<string, _File> Files { get; } = new();
    public Dictionary<string, _Directory> Directories { get; } = new();

    public IEnumerable<_Directory> AllDirectories
        => Directories.Values.Concat(Directories.Values.SelectMany(dir => dir.AllDirectories));

    private _Directory? _parent;
    public override _Directory Parent {
        get => _parent ?? Root;
        set {
            _parent?.Directories.Remove(Name);
            _parent = value!;
            _parent?.Directories.Add(Name, this);
        }
    }

    public _Directory(string name, _Directory? parent)
        : base(name, 0, parent!) { }

    public override int Size
        => Files.Values.Sum(f => f.Size) + Directories.Values.Sum(d => d.Size);
}
