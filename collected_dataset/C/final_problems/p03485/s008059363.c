#include <stdio.h>
#include <math.h>

int main(int argc, char *argv[])
{
    int a, b;
    scanf("%d %d", &a, &b);
    printf("%d\n", (int)round(0.5 * (a + b)));
    return 0;
}
