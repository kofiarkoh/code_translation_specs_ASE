#include <stdio.h>
#include <string.h>

int main(void)
{
    int n, m, p, score[1000];
    int max;
    int i, j;
    
    
    while (1){
        memset(score, 0, sizeof(score));
        scanf("%d%d", &n, &m);
        if (n == 0 && m == 0){
            break;
        }
        
        for (i = 0; i < m; i++){
            for (j = 0; j < n; j++){
                scanf("%d", &p);
                score[j] += p;
            }
        }
        max = 0;
        for (j = 0; j < n; j++){
            if (max < score[j]){
                max = score[j];
            }
        }
        printf("%d\n", max);
    }
    return (0);
}

