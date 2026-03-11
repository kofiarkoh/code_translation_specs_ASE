#include <stdio.h>
#include <inttypes.h>

int main(void)
{
    uint_fast8_t a, b;
    scanf("%" SCNuFAST8 " %" SCNuFAST8, &a, &b);
    puts((a % 3 == 0 || b % 3 == 0 || (a + b) % 3 == 0) ? "Possible" : "Impossible");
    return 0;
}
