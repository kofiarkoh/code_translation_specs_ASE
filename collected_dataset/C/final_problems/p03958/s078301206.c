/* Try AtCoder
 * author: Leonardone @ NEETSDKASU
 */
#include <stdio.h>

int dp[10001] = {1};

int abs(int a) { return a > 0 ? a : -a; }

int main(void) {
    int k, t, a, i, j, all = 1, b;
    
    scanf("%d%d", &k, &t);
    
    if (t == 1) {
        printf("%d\n", k - 1);
        return 0;
    }
    
    b = -1;
    
    for (i = 0; i < t; i++) {
        scanf("%d", &a);
        if (b < 0) b = a;
        else all &= a == b;
        for (j = 10000 - a; j >= 0; j--) {
            if (dp[j] == 0) continue;
            dp[j + a] = 1;
        }
    }
    
    if (all) {
        puts("0");
        return 0;
    }
    
    a = 10000;
    j = k / 2;
    for (i = 1; i < 10001; i++) {
        if (dp[i] == 0) continue;
        if (abs(j - i) < abs(j - a)) {
            a = i;
        }
    }
    
    if (k % 2 == 1) {
        printf("%d\n", abs(a - j) + (a != j));
    } else {
        printf("%d\n", abs(a - j));
    }

    return 0;
}