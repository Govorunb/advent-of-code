use num::Integer;

pub struct LinearEquation {
    pub a: isize,
    pub b: isize,
    pub c: isize
}

pub fn solve_system_2var(equations: &[LinearEquation; 2]) -> Option<(isize, isize)> {
    let &[
        LinearEquation {a: a1, b: b1, c: c1},
        LinearEquation {a: a2, b: b2, c: c2}
    ] = equations;

    // e.g. from the first machine in the y24d13 example:
    // (1) 94a+22b = 8400
    // (2) 34a+67b = 5400
    // multiply both sides of (2) to match its `a` coefficient with (1):
    // (1) 94a+22b = 8400
    // (2) 94a+67*(94/34)b = 5400*(94/34)
    // subtract (1) from (2):
    // ((67*94/34) - 22)b = 5400*(94/34) - 8400
    // common base of 34 (it then cancels out):
    // (67*94 - 22*34)b/34 = (5400*94 - 8400*34)/34
    // (6298-748)b = (507600-285600)
    // 5550b = 222000 => b = 40 (here, if the number is non-integer/out of bounds/etc, we can break)
    // 94a+22*40 = 8400 => a = (8400-880)/94 = 80

    // more generalized, it looks like this:
    // (B2*(A1/A2) - B1)b = C2*(A1/A2) - C1
    // ((B2*A1 - B1*A2) / A2)b = (C2*A1 - C1*A2) / A2
    // (B2*A1 - B1*A2)b = (C2*A1 - C1*A2)
    // b = (C2*A1 - C1*A2) / (B2*A1 - B1*A2)
    // a = (C1-(B1)b) / A1

    let part1 = c2*a1 - c1*a2;
    let part2 = b2*a1 - b1*a2;
    let (y, yrem) = part1.div_rem(&part2);
    if yrem != 0 {return None}

    // Ax+By=C => x=(C-By)/A
    let (x, xrem) = (c1-(b1*y)).div_rem(&a1);
    if xrem != 0 {return None}
    
    Some((x,y))
}
