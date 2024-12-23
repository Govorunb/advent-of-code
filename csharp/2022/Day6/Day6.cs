﻿using System.Diagnostics;

var testInputs = new List<string>()
{
    "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
    "bvwbjplbgvbhsrlpgdmjqwftvncz",
    "nppdvjthqldpwncqszvftbrmjlhg",
    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
};

var p1Tests = new List<int>() { 7, 5, 6, 10, 11 };
var p2Tests = new List<int>() { };

var input = File.ReadAllText("Input.txt");

int PartOne(ReadOnlySpan<char> input, int numDistinct = 4) {
    // process input here
    for (int i = numDistinct; i < input.Length;) {
        var chunk = input.Slice(i - numDistinct, numDistinct);
        var seen = new HashSet<char> { chunk[^1] };
        for (var j = chunk.Length - 2; j >= 0; j--) {
            var @char = chunk[j];
            if (seen.Contains(@char)) {
                i += j + 1;
                break;
            } else {
                seen.Add(@char);
            }
        }
        if (seen.Count == numDistinct)
            return i;
    }
    return input.Length;
}
Debug.Assert(testInputs.Zip(p1Tests).All(pair => PartOne(pair.First) == pair.Second));



// part one
Console.WriteLine(PartOne(input));

int PartTwo(ReadOnlySpan<char> input) {
    return PartOne(input, 14);
}
Debug.Assert(testInputs.Zip(p2Tests).All(pair => PartTwo(pair.First) == pair.Second));

// part two
Console.WriteLine(PartTwo(input));
