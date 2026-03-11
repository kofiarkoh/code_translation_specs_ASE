#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <inttypes.h>
#include <string.h>
#include <math.h>

intmax_t get_gcd (intmax_t a, intmax_t b)
{
    if (b == 0) return a;
    else return get_gcd(b, a % b);
}

intmax_t get_lcm (intmax_t a, intmax_t b)
{
    intmax_t g = get_gcd(a, b);
    return a / g * b;
}

int main (int argc, char const *argv[])
{
    intmax_t A, B, C, D;
    intmax_t lcm, ans = 0;

    scanf("%jd%jd%jd%jd", &A, &B, &C, &D);

    lcm = get_lcm(C, D);

    ans += B / C;
    ans += B / D;
    ans -= B / lcm;
    
    ans -= (A - 1) / C;
    ans -= (A - 1) / D;
    ans += (A - 1) / lcm;

    ans = B - A + 1 - ans;

    printf("%jd\n", ans);

    return 0;
}
