#include <stdio.h>

int main(int argc, char *argv[]) {
    int n;
    int x[100000], l[100000], a[100000];
    int i, j;
    int max = 0;
    int f;
    int amax, amaxn;
    
    scanf("%d", &n);
    for (i = 0; i < n; i++) {
        scanf("%d %d", &x[i], &l[i]);
    }
    
    while (1) {
        f = 0;
        for (i = 0; i < n; i++) {
            if (l[i] == 0) continue;
            a[i] = 0;
            for (j = 0; j < n; j++) {
                if (i == j) continue;
                if (l[j] == 0) continue;
                if (x[i] - l[i] < x[j] + l[j] && x[i] + l[i] > x[j] - l[j]) {
                    a[i]++;
                    f = 1;
                }
            }
        }
        if (f != 1) break;
        
        amax = 0;
        amaxn = 0;
        for (i = 0; i < n; i++) {
            if (amax < a[i]) {
                amax = a[i];
                amaxn = i;
            }
        }
        
        l[amaxn] = 0;
        max++;
    }
    printf("%d\n", n - max);
    return 0;
}