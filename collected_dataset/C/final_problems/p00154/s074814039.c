// AOJ Volume 1 Problem 0154 Sum of Cards

#include <stdio.h>
#include <string.h>

int now[1001], prev[1001];


void dp(int a, int b)
{
    int i, j;
    
    memcpy(prev, now, sizeof(now));
    
    for (i = 1; i <= 1000; i++){
        if (prev[i] > 0){
            for (j = 1; j <= b; j++){
                if (i + a * j <= 1000){
                    now[i + a * j] += prev[i];
                }
            }
        }
    }
    for (j = 1; j <= b; j++){
        now[a * j]++;
    }
}


int main(void)
{
    int m;
    int a, b;
    int g;
    int n;
    int i, j;
    
    while (1){
        scanf("%d", &m);
        if (m == 0){
            break;
        }

        memset(now, 0, sizeof(now));
        memset(prev, 0, sizeof(prev));
        
        for (i = 0; i < m; i++){
            scanf("%d%d", &a, &b);
            dp(a, b);
        }
        
       
        scanf("%d", &g);
        for (i = 0; i < g; i++){
            scanf("%d", &n);
            printf("%d\n", now[n]);
        }
    }
    
    return (0);
}