int Priority(char item)
    => item switch
    {
        >= 'a' and <= 'z' => 1 + (item - 'a'),
        >= 'A' and <= 'Z' => 27 + (item - 'A'),
        _ => -1,
    };

IEnumerable<char> MisplacedItems(string firstCompartment, string secondCompartment) {
    var seen = new HashSet<char>(firstCompartment);
    foreach (var item in secondCompartment) {
        if (seen.Contains(item))
            yield return item;
    }
}


var input = """
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
""";


input = File.ReadAllText("Input.txt");

var sacks = input.Split(Environment.NewLine);

var compartments = sacks.Select<string, (string first, string second)>(
    sack => (sack[..(sack.Length / 2)], sack[(sack.Length / 2)..]));

var misplacedItemsPerSack = compartments.Select(pair => MisplacedItems(pair.first, pair.second).Distinct());

var prioritiesPerSack = misplacedItemsPerSack.Select(sack => sack.Select(Priority));
var sumPrioritiesPerSack = prioritiesPerSack.Select(sack => sack.Sum());
var sumPriorities = sumPrioritiesPerSack.Sum();

Console.WriteLine(sumPriorities);




// part two
var groups = sacks.Chunk(3);
var badges = groups.Select(group => group[0].Intersect(group[1]).Intersect(group[2]).Single());
var badgePriorities = badges.Select(Priority);

var sumBadgePriorities = badgePriorities.Sum();

Console.WriteLine(sumBadgePriorities);