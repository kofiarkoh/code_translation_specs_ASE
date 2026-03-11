#include <stdio.h>
#define MAX(x,y) ((x) < (y) ? (y) : (x))
#define MIN(x,y) ((x) > (y) ? (y) : (x))
int A[19][2];
int idx[19];
int use[19];
int cash[19];
int ans = 1000000, N, swapcnt = 0;

void bfs(int x) {
    if (x == N) { 
        ans = MIN(ans, swapcnt);
        return;
    }
    for (int i = 0; i < N; i++) {
        cash[x] = A[i][(i^x)&1];
        if (use[i]) continue;
        if (x > 0 && cash[x-1] > cash[x]) continue;
        idx[x] = i;
        use[i] = 1;
        int k = 0;
        for (int j = 0; j < x; j++) if (idx[x] < idx[j]) k++;
        swapcnt += k;
        bfs(x+1);
        idx[x] = -1;
        use[i] = 0;
        swapcnt -= k;
    }
}

int main() {
    scanf("%d", &N);
    for (int i = 0; i < N; i++) scanf("%d", &A[i][0]);
    for (int i = 0; i < N; i++) scanf("%d", &A[i][1]);
    for (int i = 0; i < N; i++) idx[i] = -1;
    bfs(0);
    if (ans > 100000-1)
        printf("%d\n", -1);
    else
        printf("%d\n", ans);
    return 0;
}