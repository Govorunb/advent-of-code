using System.Drawing;

namespace Day10; 
internal enum Mnemonic {
    addx,
    noop
}

internal struct Instruction {
    public Mnemonic Mnemonic;
    public int Operand;
}

internal class CPU {
    public int X;
    public List<int> XHistory = new();
    public int Cycle;
    public Point CurrentBeamPos => BeamPosAt(Cycle);

    public static Point BeamPosAt(int cycle) {
        return new Point(cycle % 40, cycle / 40);
    }

    public bool DebugDraw;


    public CPU() {
        X = 1;
        //XHistory.Add(X);
    }

    public void AdvanceCycles(int cycles) {
        foreach (var _ in Enumerable.Range(0, cycles)) {
            Cycle++;
            XHistory.Add(X);
            if (DebugDraw) {
                Console.WriteLine($"Cycle {Cycle}");
                Console.WriteLine(CRT);
                Console.WriteLine("Finished drawing\n");
            }
        }
    }

    public int Step(Instruction instr) {
        switch (instr.Mnemonic) {
            case Mnemonic.addx:
                AdvanceCycles(2);
                X += instr.Operand;
                //XHistory[Cycle] = X;
                break;
            case Mnemonic.noop:
                AdvanceCycles(1);
                break;
        }
        return X;
    }

    public string CRT
        => string.Join('\n',
                XHistory
                    .Select((x, cycle) => Math.Abs(x - BeamPosAt(cycle).X) < 2)
                    .Chunk(40)
                    .Select(line => line.AsEnumerable().Select(doDraw => doDraw ? '#' : '.'))
                    .Select(line => new string(line.ToArray()))
                    .ToList());
}
