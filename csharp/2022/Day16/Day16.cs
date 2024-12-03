global using AoC2022.Common;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Diagnostics;
using System.Diagnostics.CodeAnalysis;
using System.Text;
using System.Text.RegularExpressions;
using QuickGraph;
using QuickGraph.Algorithms;

Dictionary<string, (int p1, int p2)> tests = new() {
    // original test case
    {"""
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
""",(
    1651, // |AA|DD|BB|JJ|HH|EE|CC
    1701
    )},
    // linear
    { """
Valve AA has flow rate=0; tunnels lead to valves BA
Valve BA has flow rate=2; tunnels lead to valves AA, CA
Valve CA has flow rate=4; tunnels lead to valves BA, DA
Valve DA has flow rate=6; tunnels lead to valves CA, EA
Valve EA has flow rate=8; tunnels lead to valves DA, FA
Valve FA has flow rate=10; tunnels lead to valves EA, GA
Valve GA has flow rate=12; tunnels lead to valves FA, HA
Valve HA has flow rate=14; tunnels lead to valves GA, IA
Valve IA has flow rate=16; tunnels lead to valves HA, JA
Valve JA has flow rate=18; tunnels lead to valves IA, KA
Valve KA has flow rate=20; tunnels lead to valves JA, LA
Valve LA has flow rate=22; tunnels lead to valves KA, MA
Valve MA has flow rate=24; tunnels lead to valves LA, NA
Valve NA has flow rate=26; tunnels lead to valves MA, OA
Valve OA has flow rate=28; tunnels lead to valves NA, PA
Valve PA has flow rate=30; tunnels lead to valves OA
""",(
    2640, // |AA|FA|GA|HA|IA|JA|KA|LA|MA|NA|OA|PA
    2670  // 1240 |AA|DA|EA|FA|GA|HA|IA|JA|CA
          // 1430 |AA|KA|LA|MA|NA|OA|PA
    )},
    // quadratic
    {"""
Valve AA has flow rate=0; tunnels lead to valves BA
Valve BA has flow rate=1; tunnels lead to valves AA, CA
Valve CA has flow rate=4; tunnels lead to valves BA, DA
Valve DA has flow rate=9; tunnels lead to valves CA, EA
Valve EA has flow rate=16; tunnels lead to valves DA, FA
Valve FA has flow rate=25; tunnels lead to valves EA, GA
Valve GA has flow rate=36; tunnels lead to valves FA, HA
Valve HA has flow rate=49; tunnels lead to valves GA, IA
Valve IA has flow rate=64; tunnels lead to valves HA, JA
Valve JA has flow rate=81; tunnels lead to valves IA, KA
Valve KA has flow rate=100; tunnels lead to valves JA, LA
Valve LA has flow rate=121; tunnels lead to valves KA, MA
Valve MA has flow rate=144; tunnels lead to valves LA, NA
Valve NA has flow rate=169; tunnels lead to valves MA, OA
Valve OA has flow rate=196; tunnels lead to valves NA, PA
Valve PA has flow rate=225; tunnels lead to valves OA
""",(
    13468, // |AA|IA|JA|KA|LA|MA|NA|OA|PA
    12887  // 4857 |AA|FA|GA|HA|IA|JA|KA|EA|DA
           // 8030 |AA|LA|MA|NA|OA|PA
    )},
    // circle
    {"""
Valve BA has flow rate=2; tunnels lead to valves AA, CA
Valve CA has flow rate=10; tunnels lead to valves BA, DA
Valve DA has flow rate=2; tunnels lead to valves CA, EA
Valve EA has flow rate=10; tunnels lead to valves DA, FA
Valve FA has flow rate=2; tunnels lead to valves EA, GA
Valve GA has flow rate=10; tunnels lead to valves FA, HA
Valve HA has flow rate=2; tunnels lead to valves GA, IA
Valve IA has flow rate=10; tunnels lead to valves HA, JA
Valve JA has flow rate=2; tunnels lead to valves IA, KA
Valve KA has flow rate=10; tunnels lead to valves JA, LA
Valve LA has flow rate=2; tunnels lead to valves KA, MA
Valve MA has flow rate=10; tunnels lead to valves LA, NA
Valve NA has flow rate=2; tunnels lead to valves MA, OA
Valve OA has flow rate=10; tunnels lead to valves NA, PA
Valve PA has flow rate=2; tunnels lead to valves OA, AA
Valve AA has flow rate=0; tunnels lead to valves BA, PA
""",(
    1288, // |AA|CA|EA|GA|IA|KA|MA|NA|OA|PA|BA
    1484  // 794 |AA|CA|EA|GA|IA|HA|FA|DA
          // 690 |AA|OA|MA|KA|JA|LA|NA|PA|BA
    )},
    // clusters
    {"""
Valve AA has flow rate=0; tunnels lead to valves AB, BB, CB
Valve AB has flow rate=0; tunnels lead to valves AA, AC
Valve AC has flow rate=0; tunnels lead to valves AB, AD
Valve AD has flow rate=0; tunnels lead to valves AC, AE
Valve AE has flow rate=0; tunnels lead to valves AD, AF
Valve AF has flow rate=0; tunnels lead to valves AE, AG
Valve AG has flow rate=0; tunnels lead to valves AF, AH
Valve AH has flow rate=0; tunnels lead to valves AG, AI
Valve AI has flow rate=0; tunnels lead to valves AH, AJ
Valve AJ has flow rate=0; tunnels lead to valves AI, AK
Valve AK has flow rate=100; tunnels lead to valves AJ, AW, AX, AY, AZ
Valve AW has flow rate=10; tunnels lead to valves AK
Valve AX has flow rate=10; tunnels lead to valves AK
Valve AY has flow rate=10; tunnels lead to valves AK
Valve AZ has flow rate=10; tunnels lead to valves AK
Valve BB has flow rate=0; tunnels lead to valves AA, BC
Valve BC has flow rate=0; tunnels lead to valves BB, BD
Valve BD has flow rate=0; tunnels lead to valves BC, BE
Valve BE has flow rate=0; tunnels lead to valves BD, BF
Valve BF has flow rate=0; tunnels lead to valves BE, BG
Valve BG has flow rate=0; tunnels lead to valves BF, BH
Valve BH has flow rate=0; tunnels lead to valves BG, BI
Valve BI has flow rate=0; tunnels lead to valves BH, BJ
Valve BJ has flow rate=0; tunnels lead to valves BI, BK
Valve BK has flow rate=100; tunnels lead to valves BJ, BW, BX, BY, BZ
Valve BW has flow rate=10; tunnels lead to valves BK
Valve BX has flow rate=10; tunnels lead to valves BK
Valve BY has flow rate=10; tunnels lead to valves BK
Valve BZ has flow rate=10; tunnels lead to valves BK
Valve CB has flow rate=0; tunnels lead to valves AA, CC
Valve CC has flow rate=0; tunnels lead to valves CB, CD
Valve CD has flow rate=0; tunnels lead to valves CC, CE
Valve CE has flow rate=0; tunnels lead to valves CD, CF
Valve CF has flow rate=0; tunnels lead to valves CE, CG
Valve CG has flow rate=0; tunnels lead to valves CF, CH
Valve CH has flow rate=0; tunnels lead to valves CG, CI
Valve CI has flow rate=0; tunnels lead to valves CH, CJ
Valve CJ has flow rate=0; tunnels lead to valves CI, CK
Valve CK has flow rate=100; tunnels lead to valves CJ, CW, CX, CY, CZ
Valve CW has flow rate=10; tunnels lead to valves CK
Valve CX has flow rate=10; tunnels lead to valves CK
Valve CY has flow rate=10; tunnels lead to valves CK
Valve CZ has flow rate=10; tunnels lead to valves CK
""",(
    2400, // |AA|CK|CX|CZ|CY|CW
    3680  // 1840 |AA|AK|AW|AX|AY|AZ
          // 1840 |AA|CK|CZ|CX|CY|CW
    )},
};

string input = """
Valve SW has flow rate=0; tunnels lead to valves LX, LD
Valve VS has flow rate=0; tunnels lead to valves JO, OO
Valve OO has flow rate=10; tunnels lead to valves KK, HD, VS, KI
Valve DZ has flow rate=8; tunnels lead to valves KV, GX, WQ, BA, PK
Valve GX has flow rate=0; tunnels lead to valves AA, DZ
Valve IF has flow rate=0; tunnels lead to valves OI, DW
Valve BO has flow rate=0; tunnels lead to valves UJ, ZT
Valve KI has flow rate=0; tunnels lead to valves OO, KU
Valve JT has flow rate=3; tunnels lead to valves FC, AM, KV, XP, XZ
Valve TQ has flow rate=0; tunnels lead to valves AA, DW
Valve KK has flow rate=0; tunnels lead to valves QW, OO
Valve NR has flow rate=0; tunnels lead to valves UG, XM
Valve VO has flow rate=0; tunnels lead to valves YR, AA
Valve MS has flow rate=17; tunnels lead to valves LT, LX
Valve JO has flow rate=0; tunnels lead to valves YR, VS
Valve ZB has flow rate=0; tunnels lead to valves UJ, LT
Valve ZT has flow rate=0; tunnels lead to valves XM, BO
Valve YR has flow rate=9; tunnels lead to valves VO, FY, WB, JO
Valve QS has flow rate=0; tunnels lead to valves QW, FY
Valve UD has flow rate=0; tunnels lead to valves CA, JB
Valve AP has flow rate=0; tunnels lead to valves CA, DW
Valve KV has flow rate=0; tunnels lead to valves JT, DZ
Valve JH has flow rate=0; tunnels lead to valves IK, UJ
Valve LD has flow rate=15; tunnels lead to valves IK, SW
Valve XK has flow rate=0; tunnels lead to valves XZ, BH
Valve XM has flow rate=11; tunnels lead to valves XP, CJ, ZT, NR
Valve FY has flow rate=0; tunnels lead to valves YR, QS
Valve GI has flow rate=22; tunnel leads to valve TI
Valve JB has flow rate=14; tunnels lead to valves WB, UD, WQ, HD
Valve DW has flow rate=6; tunnels lead to valves AP, TQ, NQ, IF, PK
Valve UJ has flow rate=13; tunnels lead to valves JH, ZB, BO
Valve KU has flow rate=0; tunnels lead to valves CA, KI
Valve WQ has flow rate=0; tunnels lead to valves JB, DZ
Valve BA has flow rate=0; tunnels lead to valves BH, DZ
Valve AA has flow rate=0; tunnels lead to valves YX, TQ, VO, GX, QP
Valve TI has flow rate=0; tunnels lead to valves GI, UG
Valve FC has flow rate=0; tunnels lead to valves QP, JT
Valve CA has flow rate=18; tunnels lead to valves KU, UD, AP
Valve QW has flow rate=25; tunnels lead to valves QS, KK
Valve XZ has flow rate=0; tunnels lead to valves JT, XK
Valve YX has flow rate=0; tunnels lead to valves AA, CJ
Valve OI has flow rate=0; tunnels lead to valves IF, BH
Valve NQ has flow rate=0; tunnels lead to valves AM, DW
Valve QP has flow rate=0; tunnels lead to valves AA, FC
Valve AM has flow rate=0; tunnels lead to valves NQ, JT
Valve XP has flow rate=0; tunnels lead to valves XM, JT
Valve BH has flow rate=12; tunnels lead to valves BA, XK, OI
Valve HD has flow rate=0; tunnels lead to valves OO, JB
Valve LT has flow rate=0; tunnels lead to valves MS, ZB
Valve LX has flow rate=0; tunnels lead to valves MS, SW
Valve CJ has flow rate=0; tunnels lead to valves XM, YX
Valve PK has flow rate=0; tunnels lead to valves DW, DZ
Valve IK has flow rate=0; tunnels lead to valves LD, JH
Valve WB has flow rate=0; tunnels lead to valves YR, JB
Valve UG has flow rate=21; tunnels lead to valves TI, NR
""";
// 1701 |AA|YR|QW|OO|JB|CA|DW|DZ|BH|JT


// common function defs

// part one
int PartOne(string input, int logLevel) {
    var puzzle = new VolcanoConundrum(input, 1);
    var preMoves = new string[] {
        //"DW",
    };
    if (logLevel > 0 && preMoves.All(puzzle.CurrentState.ValvesByName.ContainsKey)) {
        foreach (var preMove in preMoves) {
            puzzle.CurrentState.MoveAndOpen(0, preMove);
        }
    }

    var maxP = puzzle.MaxPressureForState(puzzle.CurrentState, 0);
    return maxP;
    var paths = puzzle.Solve(logLevel > 1);
    if (logLevel > 0) {
        Console.Write($"{puzzle.CurrentState.PressureReleased}\t");
        Console.WriteLine(string.Join("\n\t", paths.Select(pair => $"{pair.Key}: {string.Join("->", pair.Value)}")));
    }
    return puzzle.CurrentState.PressureReleased;
}
List<(int Expected, int Actual)> p1TestResults = tests
    .Take(99)
    .Select(pair => (pair.Value.p1, PartOne(pair.Key, 1)))
    .ToList();
Debug.Assert(p1TestResults.All(pair => pair.Expected == pair.Actual));

if (input != "real input here")
    Console.WriteLine(PartOne(input, int.MaxValue));

// part two

int PartTwo(string input, int logLevel) {
    var puzzle = new VolcanoConundrum(input, 2);
    puzzle.CurrentState.PassTime(4); // teaching the elephant
                                     // if there are several branches we can split the workload with the elephant
                                     // check out fullPath in PartOne (maybe detect "cycles" from that)

    // model it as a graph and just try to evenly split the path at AA
    var paths = puzzle.Solve(logLevel > 1);
    if (logLevel > 0) {
        Console.Write($"{puzzle.CurrentState.PressureReleased}\t");
        Console.WriteLine(string.Join("\n\t", paths.Select(pair => $"{pair.Key}: {string.Join("->", pair.Value)}")));
    }
    return puzzle.CurrentState.PressureReleased;
}

IEnumerable<(int Expected, int Actual)> p2TestResults = tests
    .Select(pair => (pair.Value.p2, PartTwo(pair.Key, 1)))
    .ToList();
Debug.Assert(p2TestResults.All(pair => pair.Expected == pair.Actual));

if (input != "real input here")
    Console.WriteLine(PartTwo(input, int.MaxValue));


[DebuggerDisplay("{ToString()}")]
readonly record struct Valve(string Name, int FlowRate, List<string> Tunnels, bool Opened = false) : IComparable<Valve> {
    public static readonly Dictionary<(string,string), List<string>> Paths = new();
    public static readonly Dictionary<(string, string), int> Costs = new();

    public Valve(Valve other) : this(other.Name, other.FlowRate, other.Tunnels, other.Opened) {
    }

    public int CompareTo(Valve other) {
        var nameComp = Name.CompareTo(other.Name);
        if (nameComp == 0)
            return Opened.CompareTo(other.Opened);
        return nameComp;
    }

    public override string ToString() {
        return $"{Name}{(Opened ? '*' : "")}";
    }
}

partial class VolcanoConundrum {
    public enum Action {
        None,
        OpenValve,
        MoveThroughTunnel,
    }

    public static Dictionary<Action, int> TimeCosts = new()
    {
        { Action.None, 0 },
        { Action.OpenValve, 1 },
        { Action.MoveThroughTunnel, 1 },
    };
    public VolcanoState CurrentState = new();

    public VolcanoConundrum(string input, int numAgents) {
        Valve.Paths.Clear();
        Valve.Costs.Clear();
        CurrentState.ValvesByName.Clear();
        foreach (var line in input.Lines()) {
            var valve = ParseLine(line);
            CurrentState.ValvesByName.Add(valve.Name, valve);
        }
        for (var i = 1; i <= numAgents; i++) {
            var agent = new VolcanoAgent(i.ToString(), "AA");
            CurrentState.Agents.Add(agent);
        }
        // make graph
        _ = CurrentState.ValveGraph;
        foreach (var emptyValve in CurrentState.Valves.Where(valve => valve.FlowRate == 0).Select(valve => valve.Name).Except("AA"))
            CurrentState.ValvesByName.Remove(emptyValve);
    }

    public IReadOnlyDictionary<string, List<string>> Solve(bool writeOutput = true) {
        Dictionary<string, List<string>> paths = CurrentState.Agents.ToDictionary(agent => agent.Name, agent => new List<string>() { agent.Location });
        while (CurrentState.TimeLeft > 0) {
            foreach (var (agent,i) in CurrentState.Agents.WithIndex().Where(pair => !pair.Element.IsBusy)) {
                var moves = MovesFor(CurrentState, agent).ToList();
                if (!moves.Any())
                    break;
                var rankedMoves = RankMoves(moves, CurrentState, agent);
                if (Debugger.IsAttached) {
                    rankedMoves = rankedMoves.ToList();
                }
                var suitableMoves = rankedMoves.Where(move => {
                    var agentIdx = CurrentState.Agents.IndexOf(agent);
                    var otherAgents = CurrentState.Agents.Except(agent);

                    // if another agent would get there quicker, let them do it instead
                    bool shouldNotTake = otherAgents.Any(otherAgent => {
                        var stateCopy = new VolcanoState(CurrentState);
                        // simulate other agent finishing their move
                        stateCopy.PassTime(otherAgent.FreeIn);
                        int otherAgentIdx = CurrentState.Agents.IndexOf(otherAgent);
                        var simOtherAgent = stateCopy.Agents[otherAgentIdx];
                        var otherMove = ScoreForMoveOpen(stateCopy, simOtherAgent, move.Valve);
                        if (otherMove.Score <= 0) // other agent can't move there
                            return false;
                        int otherTime = otherMove.TimeTaken;
                        var otherBestMove = RankMoves(MovesFor(stateCopy, simOtherAgent), stateCopy, simOtherAgent).FirstOrDefault();
                        if (otherBestMove.Score <= 0) // other agent can't move... at all
                            return false;

                        return (otherBestMove.Valve.Name == move.Valve.Name // don't take others' best nextMoves
                               && otherBestMove.Score > move.Score) // ...unless we'd get more value out of them
                            || otherTime < move.TimeTaken
                            // use turn order as the tiebreaker
                            || ((otherBestMove.Valve.Name == move.Valve.Name || otherTime == move.TimeTaken) && otherAgentIdx < agentIdx);
                    });
                    return !shouldNotTake;
                });

                MoveWithScore bestMove = suitableMoves.FirstOrDefault();
                if (bestMove == default)
                    bestMove = rankedMoves.First();

                if (!CurrentState.MoveAndOpen(i, bestMove.Valve.Name)) // didn't open all valves, want to open but don't have time
                    continue;
                paths[agent.Name].Add(agent.CurrentTarget);
            }
            CurrentState.PassTime(1, writeOutput);
        }
        return paths;
    }

    /// <summary>
    /// Simulates moving to and opening a valveName from a specific starting point, and scores the action.<br/>
    /// </summary>
    /// <param valveName="fromState">Starting state from which the open-open is simulated.</param>
    /// <param valveName="valveToOpen">Valve valveName to open, e.g. <c>"AA"</c></param>
    /// <returns>An (<see cref="int"/>, <see cref="int"/>) <see cref="Tuple{int, int}"/>.<br/>
    /// The first element is the total pressure released by <paramref valveName="valveToOpen"/> from the time it is opened until time runs out.<br/>
    /// The second element is how much time is left once the valveName is opened.
    /// </returns>
    public static MoveWithScore ScoreForMoveOpen(VolcanoState fromState, VolcanoAgent agent, Valve valveToOpen) {
        var timeCost = VolcanoState.CostToOpen(agent, valveToOpen.Name);
        bool success = timeCost > 0;
        if (!success) {
            return new(valveToOpen, int.MinValue, fromState.TimeLeft+1, -1);
        }
        int remainingTime = fromState.TimeLeft - timeCost;
        int pressureInRemainingTime = valveToOpen.FlowRate * remainingTime;
        return new(valveToOpen, pressureInRemainingTime, timeCost, remainingTime);
    }

    public int SecondOrderScore(VolcanoState fromState, VolcanoAgent agent, Valve valveToOpen) {
        var move = ScoreForMoveOpen(fromState, agent, valveToOpen);
        int agentIdx = fromState.Agents.IndexOf(agent);
        var validMove = VolcanoState.TryMoveAndOpen(fromState, agentIdx, valveToOpen, out var stateAfterMove);
        if (!validMove) return -1;
        stateAfterMove.PassTime(stateAfterMove.Agents[agentIdx].FreeIn);
        var nextMoves = MovesFor(stateAfterMove, agentIdx);
        return move.Score / nextMoves.FirstOrDefault().TimeTaken;
    }

    public double SecondOrderScore(MoveWithScore move, VolcanoState stateAfterMove, int agentIdx) {
        var nextMove = MovesFor(stateAfterMove, agentIdx).FirstOrDefault();
        if (nextMove.TimeTaken <= 0) // default/invalid
            return move.Score;
        return (double)move.Score / nextMove.TimeTaken;
    }

    static Valve ParseLine(string line) {
        var match = ValveLinePattern().Match(line);
        var name = match.Groups["valveName"].Value;
        var flowRate = int.Parse(match.Groups["flowRate"].Value);
        string[] tunnels = match.Groups["tunnels"].Value.Split(", ");
        return new(name, flowRate, tunnels.ToList());
    }

    public IEnumerable<MoveWithScore> MovesFor(VolcanoState state, int agentIdx)
        => MovesFor(state, state.Agents[agentIdx]);
    public IEnumerable<MoveWithScore> MovesFor(VolcanoState state, VolcanoAgent agent) {
        var valves = state.ValvesByName;
        var moves = valves.Values.Except(valves[agent.Location])
            // valid targets
            .Where(valve => !valve.Opened && valve.FlowRate > 0)
            // not already targeted
            .Except(state.Agents
                .SelectMany(_agent => {
                    var alreadyTargeted = _agent.Plan
                        .Select(openMove => valves[openMove.Target]);
                    if (_agent.CurrentTarget is not null &&  valves.TryGetValue(_agent.CurrentTarget, out Valve currentTarget))
                        alreadyTargeted = alreadyTargeted.Prepend(currentTarget);
                    return alreadyTargeted;
                }))
            .Select(valve => ScoreForMoveOpen(state, agent, valve))
            .Where(move => move.TimeTaken <= state.TimeLeft);
        return moves.OrderByDescending(move => move.Score)
            .Where(move => {
                var canMove = VolcanoState.TryMoveAndOpen(state, state.Agents.IndexOf(agent), move.Valve, out var moveState);
                state.PassTime(agent.FreeIn);
                return canMove && CanBeMax(moveState);
            });
    }

    public bool CanBeMax(VolcanoState? state = null) {
        VolcanoState state_ = state ?? CurrentState;
        var maxPossible = state_.PreTotal + state_.Valves.Where(valve => !valve.Opened
                                    && state_.Agents.Any(agent => VolcanoState.CostToOpen(agent, valve.Name) < state_.TimeLeft))
                                .Sum(valve => valve.FlowRate * state_.TimeLeft);
        return maxPossible >= maxSeenPressure;
    }

    private int maxSeenPressure = 0;
    private Dictionary<(MoveWithScore, MoveWithScore), int> memoizedMoveCompare = new();
    public IEnumerable<MoveWithScore> RankMoves(IEnumerable<MoveWithScore> moves, VolcanoState forState, VolcanoAgent forAgent) {
        return moves
            .OrderDescending(Comparer<MoveWithScore>.Create((first, second) => {
                if (!memoizedMoveCompare.TryGetValue((first, second), out int res)) {
                    res = CompareMoves(first, second, forState, forAgent);
                    memoizedMoveCompare[(first, second)] = res;
                }
                return res;
            })
        );
    }

    private int CompareMoves(MoveWithScore first, MoveWithScore second, VolcanoState state, VolcanoAgent agent) {
        var agentIdx = state.Agents.IndexOf(agent);

        var firstCanMove = VolcanoState.TryMoveAndOpen(state, agentIdx, first.Valve, out var firstState);
        var secondCanMove = VolcanoState.TryMoveAndOpen(state, agentIdx, second.Valve, out var secondState);
        if (!(firstCanMove && secondCanMove)) {
            // should've both been confirmed to be able to open in MovesFor but can't hurt to double check
            return firstCanMove.CompareTo(secondCanMove);
        }

        var firstSimAgent = firstState.Agents[agentIdx];
        var secondSimAgent = secondState.Agents[agentIdx];

        // simulate both versions of the agent completing the move-open
        firstState.PassTime(firstSimAgent.FreeIn);
        secondState.PassTime(secondSimAgent.FreeIn);
        return CompareStates(firstState, secondState, agentIdx);

        var movesForFirst = MovesFor(firstState, agentIdx).ToList();
        var movesForSecond = MovesFor(secondState, agentIdx).ToList();

        if (movesForFirst.Count == 0 || movesForSecond.Count == 0) {
            if (firstState.PreTotal > maxSeenPressure) {
                maxSeenPressure = firstState.PreTotal;
            } else if (secondState.PreTotal > maxSeenPressure) {
                maxSeenPressure = secondState.PreTotal;
            }
            if (movesForFirst.Count == movesForSecond.Count) {
                var pressureCompare = firstState.PressureReleased.CompareTo(secondState.PressureReleased);
                return pressureCompare == 0 ? -1 : pressureCompare;
            }
            return movesForFirst.Count.CompareTo(movesForSecond.Count);
        }
        if (movesForFirst.FirstOrDefault().Valve.Name == movesForSecond.FirstOrDefault().Valve.Name) // equivalent
            return first.Score.CompareTo(second.Score);

        MoveWithScore secondFromFirst, firstFromSecond;
        if (state.TimeLeft > 1) {
            var rankedForFirst = RankMoves(movesForFirst, firstState, firstSimAgent);
            var rankedForSecond = RankMoves(movesForSecond, secondState, secondSimAgent);
            var bestForFirst = rankedForFirst.FirstOrDefault();
            var bestForSecond = rankedForSecond.FirstOrDefault();

            secondFromFirst = bestForFirst;
            firstFromSecond = bestForSecond;
        } else {
            secondFromFirst = movesForFirst.First();
            firstFromSecond = movesForSecond.First();
            //secondFromFirst = ScoreForMoveOpen(firstState, firstSimAgent, second.Valve);
            //firstFromSecond = ScoreForMoveOpen(secondState, secondSimAgent, first.Valve);
        }

        var mutuallySwappable = secondFromFirst.Valve.Name == second.Valve.Name && firstFromSecond.Valve.Name == first.Valve.Name;
        if (mutuallySwappable || true) {
            var firstTotalTime = first.TimeTaken + secondFromFirst.TimeTaken;
            var secondTotalTime = second.TimeTaken + firstFromSecond.TimeTaken;
            var maxTime = int.Max(firstTotalTime, secondTotalTime);

            int firstRate = first.Valve.FlowRate;
            int firstNextRate = secondFromFirst.Valve.FlowRate;
            int secondRate = second.Valve.FlowRate;
            int secondNextRate = firstFromSecond.Valve.FlowRate;
            int bothRateFirst = firstRate + firstNextRate;
            int bothRateSecond = secondRate + secondNextRate;

            //            previous rate until "this" open    ->    "this" rate until "other" open     ->   "both" rate until common end
            int firstAddedPressure = 0 * first.TimeTaken + firstRate * secondFromFirst.TimeTaken + bothRateFirst * (maxTime - firstTotalTime);
            int secondAddedPressure = 0 * second.TimeTaken + secondRate * firstFromSecond.TimeTaken + bothRateSecond * (maxTime - secondTotalTime);

            return firstAddedPressure.CompareTo(secondAddedPressure);
        }
        //var firstScore2 = SecondOrderScore(first, firstState, agentIdx);
        //var secondScore2 = SecondOrderScore(second, secondState, agentIdx);
        //return firstScore2.CompareTo(secondScore2);
        //return first.Score.CompareTo(second.Score);
        
        return (first.Score + movesForFirst.Average(move => move.Score)).CompareTo(second.Score + movesForSecond.Average(move => move.Score));
    }

    private readonly Dictionary<(VolcanoState first, VolcanoState second), int> memoizedStateCompare = new();
    private readonly ConcurrentDictionary<VolcanoState, int> stateMaxPressure = new();

    public int CompareStates(VolcanoState first, VolcanoState second, int agentIdx) {
        return memoizedStateCompare.GetOrAdd((first, second), () => MaxPressureForState(first, agentIdx).CompareTo(MaxPressureForState(second, agentIdx)));
    }

    public int MaxPressureForState(VolcanoState state, int agentIdx) {
        return stateMaxPressure.GetOrAdd(state, (_) => {
            var moves = MovesFor(state, agentIdx);
            int maxP = state.PreTotal;
            moves.AsParallel().ForAll((move) => {
                if (!VolcanoState.TryMoveAndOpen(state, agentIdx, move.Valve, out var moveState))
                    return;
                moveState.PassTime(move.TimeTaken);
                var moveP = MaxPressureForState(moveState, agentIdx);
                lock (moves) {
                    if (maxP < moveP)
                        maxP = moveP;
                }
            });
            return maxP;
        });
    }
    
}

[DebuggerDisplay("{ToString()}")]
class VolcanoAgent {
    public string Name;
    public string Location;
    public string CurrentTarget;
    public int TimeToTarget;
    // 1-timeCost actions only
    public readonly Queue<(string Target, int Time)> Plan = new();

    public string Goal => Plan.LastOrDefault().Target;
    public bool IsBusy => TimeToTarget > 0;
    public int FreeIn => Plan.Sum(move => move.Time) + TimeToTarget;

    public VolcanoAgent(string name, string location) : this(name, location, location) { }
    public VolcanoAgent(string name, string location, string? currentTarget = null){
        Name = name;
        Location = location;
        CurrentTarget = currentTarget ?? location;
    }

    public VolcanoAgent(VolcanoAgent other) : this(other.Name, other.Location, other.CurrentTarget) {
        foreach (var plannedAction in other.Plan)
            Plan.Enqueue(plannedAction);
    }

    internal void ActivateNextTask() {
        Plan.TryDequeue(out var nextMove);
        (CurrentTarget, TimeToTarget) = nextMove;
    }

    public override string ToString() {
        return $"VolcanoAgent {{{Name}, {Location}}}";
    }
}

[DebuggerDisplay("{ToString()}")]
record struct VolcanoState : IEquatable<VolcanoState> {
    public Dictionary<string, Valve> ValvesByName = new();
    public List<Valve> Valves => ValvesByName.Values.ToList();
    
    private AdjacencyGraph<string, Edge<string>>? _graph;
    public AdjacencyGraph<string, Edge<string>> ValveGraph
        => _graph ??= MakeGraph();
    
    public int TimeLeft { get; private set; } = 30;
    public List<VolcanoAgent> Agents = new();
    public int PressureReleased { get; private set; }

    public int PressurePerMinute => Valves.Where(valve => valve.Opened).Sum(valve => valve.FlowRate);

    public int PreTotal => PressureReleased + PressurePerMinute * TimeLeft;

    public bool Equals(VolcanoState other) {
        return TimeLeft.Equals(other.TimeLeft)
            && Valves.Zip(other.Valves).All((pair) => pair.First.Equals(pair.Second))
            && Agents.Zip(other.Agents).All(pair => pair.First.Location.Equals(pair.Second.Location));
    }

    public override int GetHashCode() {
        return TimeLeft << 28
            + Agents.Sum(agent => agent.Location.GetHashCode())
            + Valves.Select(valve => valve.Opened).Aggregate(0, (acc, opened) => acc << 1 + (opened ? 1 : 0));
    }

    public VolcanoState() {
        // why does the compiler not generate a warning when this is removed
        // you can still call new() on a struct even if it's not defined
    }

    public VolcanoState(VolcanoState other) {
        PressureReleased = other.PressureReleased;
        _graph = other.ValveGraph;
        ValvesByName.AddRange(other.ValvesByName
            //.Select((pair) => (pair.Key, pair.Value))
            );
        TimeLeft = other.TimeLeft;
        Agents.AddRange(other.Agents.Select(agent => new VolcanoAgent(agent)));
    }

    private AdjacencyGraph<string, Edge<string>> MakeGraph() {
        var graph = new AdjacencyGraph<string, Edge<string>>();
        var valves = ValvesByName;
        
        graph.AddVerticesAndEdgeRange(Valves.SelectMany(valve => valve.Tunnels.Select(tunnel => new Edge<string>(valve.Name, valves[tunnel].Name))));

        foreach (var start in valves.Values) {
            var tryFunc = graph.ShortestPathsDijkstra(edge => 1, start.Name);
            foreach (var end in Valves.Except(start)) {
                var path = Enumerable.Empty<Edge<string>>();
                tryFunc(end.Name, out path);
                Valve.Paths[(start.Name, end.Name)] = path.Select(edge => edge.Target).ToList();
                Valve.Costs[(start.Name, end.Name)] = path.Count();
            }
        }

        return graph;
    }

    public void PassTime(int deltaT = 1, bool printOutput = false) {
        if (printOutput)
            Console.WriteLine($"{deltaT} {(deltaT == 1 ? "minute passes" : "minutes pass")}...");
        for (var i = 0; i < deltaT && TimeLeft > 0; i++) {
            var openedValves = Valves.Where(valve => valve.Opened);
            var pressureReleased = openedValves.Sum(valve => valve.FlowRate);
            PressureReleased += pressureReleased;

            if (printOutput) {
                Console.WriteLine($"=== Minute {31-TimeLeft} ===");
                Console.WriteLine(openedValves.Any() ?
                    $"Open valves: {string.Join(',', openedValves.Select(valve => valve.Name))}"
                    : $"No valves are open.");
                if (pressureReleased == 0) {
                    Console.WriteLine($"The valves release no pressure.");
                } else if (pressureReleased < 0) {
                    Console.WriteLine($"The valves gain {-pressureReleased} pressure???");
                } else {
                    if (openedValves.Count() > 1) {
                        Console.WriteLine($"The valves release {string.Join('+', openedValves.Select(valve => valve.FlowRate))}={pressureReleased} pressure.");
                    } else {
                        Console.WriteLine($"The valve releases {pressureReleased} pressure.");
                    }
                }
            }

            foreach (var agent in Agents) {
                if (!agent.IsBusy && agent.Plan.Any())
                    agent.ActivateNextTask(); // not *really* idle
                if (printOutput) {
                    var reportSb = new StringBuilder($"Agent {agent.Name} (at {agent.Location}) is ");
                    // action
                    reportSb.Append(agent.CurrentTarget switch
                    {
                        null => "idle",
                        var target when target == agent.Location => $"opening valve {target}",
                        _ => $"moving towards {agent.CurrentTarget}",
                    });

                    reportSb.Append('.');
                    Console.WriteLine(reportSb.ToString());
                }
            }

            PerformAgentActions();
            TimeLeft--;
            if (printOutput) {
                Console.WriteLine($"{TimeLeft} {(TimeLeft == 1 ? "minute" : "minutes")} left.");
                Console.WriteLine('\n');
            }
        }
    }

    private void PerformAgentActions() {
        foreach (var agent in Agents.Where(agent => agent.IsBusy)) {
            switch (agent.CurrentTarget) {
                case null:
                    break;
                case var target when target == agent.Location:
                    var valve = ValvesByName[target];
                    if (valve.Opened)
                        throw new Exception("tried to open an already open valve");
                    ValvesByName[valve.Name] = valve with { Opened = true };
                    agent.TimeToTarget--;
                    break;
                default:
                    var currValve = ValvesByName[agent.Location];
                    if (agent.TimeToTarget <= 2)
                        agent.Location = agent.CurrentTarget;
                    agent.TimeToTarget--;
                    break;
            }
            if (agent.TimeToTarget == 0)
                agent.ActivateNextTask();
        }
    }

    public static int CostToOpen(VolcanoAgent agent, string valve) 
        => CostToOpen(agent.Location, valve);

    public static int CostToOpen(string fromValve, string toValve) {
        var canOpen = Valve.Costs.TryGetValue((fromValve, toValve), out int cost);
        if (!canOpen) return -1;
        cost *= VolcanoConundrum.TimeCosts[VolcanoConundrum.Action.MoveThroughTunnel];
        return cost+1;
    }

    /// <summary>
    /// Simulates moving to a valveName and opening it.
    /// </summary>
    /// <param valveName="valveName"><inheritdoc cref="MoveMulti(string)"/></param>
    /// <returns>A <see cref="bool"/> indicating whether or not the open-open was successful.<br/>
    /// This can fail if the target valveName is not reachable from the current valveName, or if there's not enough time to perform the open.
    /// </returns>
    public bool MoveAndOpen(int agentIdx, string targetValve) {
        var agent = Agents[agentIdx];
        if (Valve.Costs.TryGetValue((agent.Location, targetValve), out var cost) && TimeLeft >= cost) {
            agent.Plan.Enqueue((targetValve, cost+1));
            if (!agent.IsBusy)
                agent.ActivateNextTask();
            return true;
        }
        return false;
    }

    public static bool TryMoveAndOpen(VolcanoState fromState, int agentIdx, Valve valve, out VolcanoState resultingState) {
        resultingState = new VolcanoState(fromState);
        return resultingState.MoveAndOpen(agentIdx, valve.Name);
    }

    public override string ToString() {
        return $"VolcanoState {{ {TimeLeft}, [{string.Join(", ", Agents)}] }}";
    }
}

partial class VolcanoConundrum {
    [GeneratedRegex(@"^Valve (?<valveName>\w+) has flow rate=(?<flowRate>\d+); tunnels? leads? to valves? (?<tunnels>.*)$")]
    private static partial Regex ValveLinePattern();
}

internal record struct MoveWithScore(Valve Valve, int TotalPressureFromValve, int TimeTaken, int TimeLeft) {
    public int Score => TotalPressureFromValve / TimeTaken;
}