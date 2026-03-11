#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[])
{
    int A, B, C;
    int ab = 0;
    int ac = 0;
    int bc = 0;

    if (scanf("%d %d %d", &A, &B, &C) < 3)
    {
        exit(1);
    }

    if (A < 1 && A > 100 && B < 1 && B > 100 && C < 1 && C > 100)
    {
        exit(1);
    }

    ab = A - B;
    if (ab < 0)
    {
        ab = -ab;
    }

    ac = A - C;
    if (ac < 0)
    {
        ac = -ac;
    }

    bc = B - C;
    if (bc < 0)
    {
        bc = -bc;
    }

    if (ac <= ab && bc <= ab)
    {
        printf("%d\n", ac + bc);
    }
    else if (ab <= ac && bc <= ac)
    {
        printf("%d\n", ab + bc);
    }
    else
    {
        printf("%d\n", ab + ac);
    }
    return 0;
}