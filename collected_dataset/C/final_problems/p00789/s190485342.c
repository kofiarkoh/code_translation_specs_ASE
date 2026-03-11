#include <stdio.h>
#include <string.h>

int main(void)
{
    int sq[20];
    int dp[301];
    int n;
    int i, j;
    
    for (i = 0; i < 17; i++){
        sq[i] = (i + 1) * (i + 1);
    }
    
    memset(dp, 0, sizeof(dp));
    dp[0] = 1;
    
    for (i = 0; i < 17; i++){
        for (j = 0; j + sq[i] <= 300; j++){
            if (dp[j] != -1){
                dp[j + sq[i]] += dp[j];
            }
        }
    }
    
    while (1){
        scanf("%d", &n);
        
        if (n == 0){
            break;
        }
        
        printf("%d\n", dp[n]);
    }
    
    return (0);
}