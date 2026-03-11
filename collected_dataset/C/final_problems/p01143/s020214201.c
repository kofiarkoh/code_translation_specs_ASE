#include <stdio.h>

int main(void)
{
    int n, m, p;

    while (scanf("%d %d %d", &n, &m, &p), n || m || p){
        int x[128];
        int i;
        int sum = 0;

        for (int i = 0; i < n; i++){
            scanf("%d", x + i);
            sum += x[i];
        }
        printf("%d\n", x[m - 1] ? sum * (100 - p) / x[m - 1] : 0);
    }

    return 0;
}