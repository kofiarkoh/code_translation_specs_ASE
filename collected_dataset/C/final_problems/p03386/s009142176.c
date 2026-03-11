#include <stdio.h>
#include <stdlib.h>

int main()
{
    int A, B, K, i;
    int output[200];
    for (i = 0; i < 200; ++i)
    {
        output[i] = 0;
    }
    if(scanf("%d", &A) < 0) exit(1);
    if(scanf("%d", &B) < 0) exit(1);
    if(scanf("%d", &K) < 0) exit(1);
    int x, y;
    for (i = 0; i < K; ++i)
    {
        x = A + i;
        y = B - i;
        if (x >= y) {
            output[i] = x;
            break;
        } else if (x+1 == y) {
            output[i] = x;
            output[200 - i - 1] = y;
            break;
        } else {
            output[i] = x;
            output[200 - i - 1] = y;
        }
    }
    for (i = 0; i < 200; ++i)
    {
        if (output[i] != 0)
        {
            printf("%d\n", output[i]);
        }
    }

    return 0;
}