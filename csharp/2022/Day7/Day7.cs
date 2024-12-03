global using AoC2022.Common;
using Day7;
using System.Diagnostics;

string testInput = """
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
""";

int p1TestOutput = 95437;

string input = File.ReadAllText("Input.txt");

_Directory ProcessLine(_Directory context, string line) {
    if (line[0] == '$')
        return ProcessCommand(context, line);
    else {
        ProcessOutput(context, line);
        return context;
    }
}

_Directory ProcessCommand(_Directory context, string line) {
    if (line.StartsWith("$ cd")) {
        var target = line[5..].Split('/');
        foreach (var dir in target) {
            if (dir.Equals("..", StringComparison.Ordinal))
                context = context == _Directory.Root ? _Directory.Root : context.Parent;
            else
                context = context.Directories.GetOrAdd(dir, () => new _Directory(dir, context));
        }
    }
    // ignore $ ls
    return context;
}

void ProcessOutput(_Directory context, string line) {
    var lineSplit = line.Split(' ', 2);
    (string sizeOrDir, string name) = (lineSplit[0], lineSplit[1]);
    if (sizeOrDir.Equals("dir")) {
        _ = new _Directory(name, context);
    } else {
        _ = new _File(name, int.Parse(sizeOrDir), context);
    }
}

void ProcessInput(string input) {
    var lines = input.Split(Environment.NewLine);
    var context = _Directory.Root;
    foreach (var line in lines) {
        context = ProcessLine(context, line);
    }
}

// part one
int PartOne(string input) {
    _Directory.Root.Directories.Clear();
    _Directory.Root.Files.Clear();
    ProcessInput(input);

    var targetDirs = _Directory.Root.AllDirectories
        .Where(dir => dir.Size <= 100000);
    return targetDirs.Sum(d => d.Size);
}

Debug.Assert(PartOne(testInput) == p1TestOutput);

Console.WriteLine(PartOne(input));

// part two
int p2TestOutput = 24933642;

int PartTwo(string input) {
    _Directory.Root.Directories.Clear();
    _Directory.Root.Files.Clear();
    ProcessInput(input);

    const int spaceTotal = 70000000;
    const int targetFreeSpace = 30000000;
    const int targetUsedSpace = spaceTotal - targetFreeSpace;
    var spaceUsed = _Directory.Root.Size;

    var needed = spaceUsed - targetUsedSpace;

    var sorted = _Directory.Root.AllDirectories
        .OrderBy(dir => dir.Size);

    return sorted.First(dir => dir.Size >= needed).Size;
}

Debug.Assert(PartTwo(testInput) == p2TestOutput);

Console.WriteLine(PartTwo(input));
