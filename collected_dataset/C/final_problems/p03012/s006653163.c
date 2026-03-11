#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>
int main(void)
{
    int N, all = 0, S1 = 0, S2, ans = 1000000;
    scanf("%d", &N);
    int W[N];
    for (int i = 0; i < N; i++) {
        scanf("%d", &W[i]);
    } 
    for (int k = 0; k < N; k++) {
            all += W[k];
        }
    for (int i = 0; i < N; i++) {
        S1 += W[i];
        S2 = all - S1;
        if (ans > abs(S2 - S1)) {
            ans = abs(S2 - S1);
        }
    }
    printf("%d\n", ans);
    return 0;
}