// See https://aka.ms/new-console-template for more information

bool IsSubRange(Range maybeSuper, Range maybeSub) {
    //if (maybeSuper.Start.IsFromEnd || maybeSuper.End.IsFromEnd
    //    || maybeSub.Start.IsFromEnd || maybeSub.End.IsFromEnd)
    //    return false;
    return maybeSuper.Start.Value <= maybeSub.Start.Value
        && maybeSuper.End.Value >= maybeSub.End.Value;
}

bool IsOverlap(Range first, Range second) {
    var firstRange = Enumerable.Range(first.Start.Value, first.End.Value + 1 - first.Start.Value);
    var secondRange = Enumerable.Range(second.Start.Value, second.End.Value + 1 - second.Start.Value);
    return firstRange.Intersect(secondRange).Any();
}

var input = """
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
""";

input = File.ReadAllText("Input.txt");

var assignments = input.Split(Environment.NewLine);

var pairs = assignments.Select(assignment => assignment.Split(','));

var sections = pairs.Select(
    pair => pair.Select(sections => sections.Split('-').Select(int.Parse).ToList())
        .Select<List<int>, Range>(sectionIDs => new(sectionIDs[0], sectionIDs[1]))
        .ToList())
    .Select(pair => (pair[0], pair[1]));

var subranges = sections.Where(pair => IsSubRange(pair.Item1, pair.Item2) || IsSubRange(pair.Item2, pair.Item1));

Console.WriteLine(subranges.Count());

// part two
var overlaps = sections.Where(pair => IsOverlap(pair.Item1, pair.Item2));

Console.WriteLine(overlaps.Count());