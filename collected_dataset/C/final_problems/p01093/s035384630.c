#include <stdio.h>
#include <stdlib.h>

int min(int x, int y) {
    return (x > y ? y : x);
}

int main() {
    int n, a[1001];
    int ans;

    while (1) {
        scanf("%d", &n);
        if (n == 0) { break; }

        for (int i = 0; i < n; i++) { scanf("%d", &a[i]); }

        ans = 1e8;

        for (int i = 0; i < n-1; i++) {
            for (int j = i+1; j < n; j++) {
                ans = min(ans, abs(a[i] - a[j]));
            }
        }
        printf("%d\n", ans);
    }
    return 0;
}
