#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>

int main()
{
    int x, y, result = 0;

    scanf("%d %d", &x, &y);

    switch (x)
    {
    case 1:
        result += 300000;
        break;
    case 2:
        result += 200000;
        break;
    case 3:
        result += 100000;
        break;
    default:
        break;
    }

    switch (y)
    {
    case 1:
        result += 300000;
        break;
    case 2:
        result += 200000;
        break;
    case 3:
        result += 100000;
        break;
    default:
        break;
    }

    if( x == 1 && y == 1 )
        result += 400000;

    printf("%d\n", result);
    return 0;
}