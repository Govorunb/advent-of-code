using System.Diagnostics.CodeAnalysis;
using System.Text.RegularExpressions;

namespace Day11; 
public enum Operator { Add, Subtract, Multiply, Divide, Square }
internal class Monkey {
    public static List<Monkey> Monkeys = new();
    public static long LCM => Monkeys.Select(monkey => monkey.Divisor).Aggregate(1L, (acc, divisor) => acc * divisor);

    public int Index;
    public Queue<long> Items = new();

    public int ItemsInspected;

    public Operator Operator;
    public int? Operand;
    [MemberNotNullWhen(false, nameof(Operand))]
    public bool OperandIsSelf => Operand is null;
    public long Divisor;
    private int _throwToIfTrue;
    private int _throwToIfFalse;
    public Monkey ThrowToIfTrue => Monkeys[_throwToIfTrue];
    public Monkey ThrowToIfFalse => Monkeys[_throwToIfFalse];


    public Monkey(Operator @operator, int? operand, long divisor, int throwIfTrue, int throwIfFalse) {
        Monkeys.Add(this);
        Index = Monkeys.Count - 1;
        Operator = @operator;
        Operand = operand;
        Divisor = divisor;
        _throwToIfTrue = throwIfTrue;
        _throwToIfFalse = throwIfFalse;
    }

    public void Turn(bool relief = true) {
        while (Items.TryDequeue(out long item)) {
            var worry = InspectItem(item, relief);
            ThrowItem(worry);
        }
    }

    public long InspectItem(long item, bool relief) {
        ItemsInspected++;
        return relief ? Operation(item) / 3
            : Operation(item);
    }

    public long Operation(long item) {
        var operand = Operand ?? item;
        return Operator switch
        {
            Operator.Add => item + operand,
            Operator.Subtract => item - operand,
            Operator.Multiply => item * operand,
            Operator.Divide => item / operand,
            Operator.Square => item * item,
            _ => item,
        } % LCM;
    }

    public void ThrowItem(long item) {
        if (item % Divisor == 0)
            ThrowToIfTrue.Items.Enqueue(item);
        else
            ThrowToIfFalse.Items.Enqueue(item);
    }

    public static Monkey FromTextDef(string textDef) {
        var lines = textDef.Split(Environment.NewLine).Select(line => line.Split(": ")[^1]).ToArray();
        if (!lines[0].Matches(/* lang=regex */ @"^Monkey \d:")) {
            return null!;
        }
        // starting startingItems
        long[] startingItems = lines[1].Split(", ").Select(long.Parse).ToArray();

        // test
        var divisor = int.Parse(lines[3][lines[3].LastIndexOf(' ')..]);

        // operation
        var operationMatch = new Regex(@"new = old (?<operator>.) (?<operand>.+)", RegexOptions.CultureInvariant | RegexOptions.Compiled)
            .Match(lines[2]);
        int? operand = null;
        if (int.TryParse(operationMatch.Groups["operand"].Value, out int constOperand))
            operand = constOperand;
        var operatorChar = operationMatch.Groups["operator"].Value[0];
        Operator @operator = operatorChar switch
        {
            '+' => Operator.Add,
            '-' => Operator.Subtract,
            '*' when operand is null => Operator.Square,
            '*' => Operator.Multiply,
            '/' => Operator.Divide,
            _ => throw new NotImplementedException(),
        };

        // throws
        int throwIfTrue = int.Parse(lines[4][lines[4].LastIndexOf(' ')..]);
        int throwIfFalse = int.Parse(lines[5][lines[5].LastIndexOf(' ')..]);

        var monkey = new Monkey(@operator, operand, divisor, throwIfTrue, throwIfFalse);
        foreach (var startingItem in startingItems)
            monkey.Items.Enqueue(startingItem);
        return monkey;
    }
}
