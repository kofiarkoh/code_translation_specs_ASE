#include <stdio.h>
 
int main(void)
{
    int i, m, n, sum, f;
    int d[1000];
     
    while (scanf("%d %d", &n, &m), m + n != 0){
        for (i = 0; i < n; i++){
            d[i] = 1;
        }
         
        i = sum = f = 0;
        while (1){
            sum += d[i];
            if (sum == m){
                d[i] = 0;
                sum = 0;
                f++;
            }
            if (f == n) break;
            if (i == n - 1){
                i = 0;
                continue;
            }
            i++;
        }
        printf("%d\n", i + 1);
    }
     
    return 0;
}