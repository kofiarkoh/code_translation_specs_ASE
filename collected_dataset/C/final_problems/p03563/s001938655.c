#include <stdio.h>
#include <string.h>

int performanceCheck(int a, int b)
{
    int performance = 0;
    performance = b * 2 - a;
    return performance;
}

int main(int argc, char *argv[])
{
    int R = 0;
    do
    {
        scanf("%d", &R);
    } while (R < 0 || 4500 < R);

    int G = 0;
    do
    {
        scanf("%d", &G);
    } while (G < 0 || 4500 < G);

    printf("%d\n", performanceCheck(R, G));
}