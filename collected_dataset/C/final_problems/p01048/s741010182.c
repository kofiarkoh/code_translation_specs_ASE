#include <stdio.h>

int main(void)
{
    int N, n;
    int i, j;
    
    scanf("%d", &N);
    i = 1;
    while (1){
        n = 0;
        for (j = 1; j <= i; j++){
            if (i % j == 0){
                n++;
            }
        }
        if (n == N){
            break;
        }
        i++;
    }
    
    printf("%d\n", i);
    
    return (0);
}

