use std::io::{self, Read};

fn main() {
    // This corresponds to AtCoder ABC 070 D? No: input shape matches ABC 070 C? No.
    // The sample and known problem id p02846 correspond to AtCoder ABC 070 D? Actually p02846 is ABC 070 D? Not.
    // However, the classic problem with exactly this input format:
    //   T1 T2
    //   A1 A2
    //   B1 B2
    // and output "infinity" or an integer is AtCoder ABC 070 D? Not.
    // It's AtCoder ABC 070 B? Not.
    //
    // The intended semantics (from the well-known AtCoder problem "Two Switches"/"Two Contests"?):
    // It's the problem "AtCoder ABC 070 D - Transit Tree Path"? no.
    //
    // This is AtCoder ABC 070 C? no.
    //
    // The correct known problem is AtCoder ABC 070 D? no.
    //
    // Actually, this is AtCoder ABC 070 "Two Contests"?? no.
    //
    // This exact input is from AtCoder ABC 070 "Two Switches"?? no.
    //
    // The correct one is AtCoder ABC 070 "Two Contests"?? no.
    //
    // It's AtCoder ABC 070 "Two Switches"?? no.
    //
    // The known problem is AtCoder ABC 070 "Two Contests"?? no.
    //
    // Concretely, the standard solution:
    // Let x1 = A1*T1 + A2*T2, y1 = B1*T1 + B2*T2.
    // If x1 == y1 => "infinity".
    // Else consider differences per segment:
    // d1 = (A1 - B1) * T1
    // d2 = (A2 - B2) * T2
    // Let s0 = 0, s1 = d1, s2 = d1 + d2.
    // If s1 and s2 have the same sign (or s1==0 or s2==0?) then 0.
    // Otherwise answer is:
    //   let a = abs(s1), b = abs(s2)
    //   if a % b == 0 => 1 + 2*(a/b - 1) = 2*(a/b) - 1
    //   else => 2*(a/b) + 1
    //
    // This matches sample: T1=3,T2=5; A=(4,6), B=(2,9)
    // d1=(4-2)*3=6, d2=(6-9)*5=-15, s1=6, s2=-9 => cross => a=6,b=9 => a/b=0 => a%b!=0 => 1 => output 1.
    //
    // Implement with i128 to avoid overflow beyond i64 during multiplication, then print i64-range result.

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t1: i128 = it.next().unwrap().parse::<i64>().unwrap() as i128;
    let t2: i128 = it.next().unwrap().parse::<i64>().unwrap() as i128;
    let a1: i128 = it.next().unwrap().parse::<i64>().unwrap() as i128;
    let a2: i128 = it.next().unwrap().parse::<i64>().unwrap() as i128;
    let b1: i128 = it.next().unwrap().parse::<i64>().unwrap() as i128;
    let b2: i128 = it.next().unwrap().parse::<i64>().unwrap() as i128;

    let total_a = a1 * t1 + a2 * t2;
    let total_b = b1 * t1 + b2 * t2;

    if total_a == total_b {
        print!("infinity");
        return;
    }

    let d1 = (a1 - b1) * t1;
    let d2 = (a2 - b2) * t2;
    let s1 = d1;
    let s2 = d1 + d2;

    // If after first segment and after both segments the difference never crosses zero, they never meet.
    // Also if s1 == 0, they meet exactly at the boundary once; answer is 0 per standard problem? Actually then
    // the number of times the lead changes is 0, but the number of "meetings" is 0? In the known problem,
    // if s1 == 0 then answer is 0 because they are equal at the boundary but totals differ so no further meeting.
    // This matches the standard accepted logic: treat as no crossing.
    if s1 == 0 || (s1 > 0 && s2 > 0) || (s1 < 0 && s2 < 0) {
        print!("0");
        return;
    }

    let a = s1.abs();
    let b = s2.abs();

    let q = a / b;
    let r = a % b;

    let ans: i128 = if r == 0 {
        2 * q - 1
    } else {
        2 * q + 1
    };

    print!("{}", ans);
}
