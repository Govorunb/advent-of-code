Dictionary<char, int> map = new()
{
    { 'A', 0 },
    { 'B', 1 },
    { 'C', 2 },
    { 'X', 0 },
    { 'Y', 1 },
    { 'Z', 2 },
};

int ShapeIndex(char shape)
    => map[shape];

int ShapeScore(int shapeIndex)
    => 1 + shapeIndex;

int Outcome(int oppShapeIdx, int ourShapeIdx)
    => oppShapeIdx == ourShapeIdx ? 0
        : ((oppShapeIdx + 1) % 3 == ourShapeIdx ? 1
            : -1);

int RoundScore(int oppShapeIdx, int ourShapeIdx)
    => ShapeScore(ourShapeIdx) + 3 * (1 + Outcome(oppShapeIdx, ourShapeIdx));

int OppRoundScore(int oppShapeIdx, int ourShapeIdx)
    => RoundScore(ourShapeIdx, oppShapeIdx);


var input = """
A Y
B X
C Z
""";

input = File.ReadAllText("Input.txt");

var roundsEnc = input.Split(Environment.NewLine);
var roundsHandsEnc = roundsEnc.Select(round => round.Split(' ').Select(hand => hand.Single()).ToList());
var roundsDec = roundsHandsEnc.Select(round => round.Select(hand => ShapeIndex(hand)).ToList())
    .Select(round => (round[0], round[1]));

var roundScores = roundsDec.Select(round => RoundScore(round.Item1, round.Item2));

Console.WriteLine(roundScores.Sum());


// part two
map['X'] = -1;
map['Y'] = 0;
map['Z'] = 1;
int ShapeIndexFromOutcome(int oppShapeIndex, int desiredOutcome)
    => Enumerable.Range(0, 3).First(i => Outcome(oppShapeIndex, i) == desiredOutcome);

var roundsWithOutcome = roundsHandsEnc.Select(round => (ShapeIndex(round[0]), ShapeIndex(round[1])));

roundsDec = roundsWithOutcome.Select(round => (round.Item1, ShapeIndexFromOutcome(round.Item1, round.Item2)));

roundScores = roundsDec.Select(round => RoundScore(round.Item1, round.Item2));

Console.WriteLine(roundScores.Sum());