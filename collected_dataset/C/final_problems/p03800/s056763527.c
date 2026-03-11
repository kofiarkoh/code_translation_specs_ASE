#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    int i, N, W;
    char S[100010];
    scanf("%d%s", &N, &S);

    W = 0;
    for (i=0;i<N;i++) if (S[i] == 'x') W++;
    if (W%2) printf("-1\n");
    else {
        W = 0;
        for (i=0;i<N;i++) {
            if (!W && S[i] == 'o') printf("S");
            else if (!W && S[i] == 'x') {
                printf("S");
                W = 1;
            } else if (W && S[i] == 'o') printf("W");
            else if (W && S[i] == 'x') {
                printf("S");
                W = 0;
            }
        }
        printf("\n");
    }

    return 0;
}