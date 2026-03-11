#include <stdio.h>
#include <string.h>

int main(void) {
    int i, j, N, x[510], y[510], z[250010] = {0}, w[250010] = {0}, w1[250010] = {0};
    scanf("%d", &N);
    for (i=1;i<=N;i++) scanf("%d", &x[i]);

    for (i=1;i<=N;i++) {
        z[x[i]] = i;
        w[i]++;
    }
    for (j=x[1]+1;w[1]<N;j++) {
        if (!z[j]) {
            z[j] = 1; w[1]++;
        }
    }
    for (i=2;i<=N;i++) {
        for (j=1;w[i]<N;j++) {
            if (!z[j]) {
                z[j] = i; w[i]++;
            }
        }
    }
    for (i=1;i<=N;i++) {
        for (j=1;j<=N*N && w[i] < N;j++) {
            if (!z[j]) {
                z[j] = i; w[i]++;
            }
        }
    }
    for (j=1;j<=N*N;j++) {
        w1[z[j]]++;
        if (w1[z[j]] == z[j]) y[z[j]] = j;
    }
    for (i=1;i<=N;i++) {
        if (y[i] != x[i]) {
            printf("No\n");
            return 0;
        }
    }
    printf("Yes\n");
    for (j=1;j<=N*N;j++) {
        printf("%d", z[j]);
        if (j<N*N) printf(" ");
        else printf("\n");
    }
    return 0;
}