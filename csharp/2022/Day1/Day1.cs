var input = """
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
""";

input = File.ReadAllText("Input.txt");

var elfStrings = input.Split(Environment.NewLine + Environment.NewLine);
var elfValues = elfStrings.Select(elf => elf
        .Split(Environment.NewLine)
        .Select(line => int.Parse(line)));
var elfSums = elfValues.Select(vals => vals.Sum());

var maxVals = elfSums
    .OrderDescending();

// part one
Console.WriteLine(maxVals.First());

// part two
int numTop = 3;
Console.WriteLine(maxVals.Take(numTop).Sum());
