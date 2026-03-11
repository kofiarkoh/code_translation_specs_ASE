#include <stdio.h>

int main()
{
    int R, G, B, N;
    scanf("%d %d %d %d", &R, &G, &B, &N);

    int numOfSets = 0;
    for (int r = 0; r <= N; r += R)
    {
        for (int rg = r; rg <= N; rg += G)
        {
            if ((N - rg) % B == 0)
            {
                numOfSets++;
            }
        }
    }
    printf("%d\n", numOfSets);

    return 0;
}
