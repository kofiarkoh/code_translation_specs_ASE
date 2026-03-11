#include <stdio.h>

int main(void)
{
    int n;
    int start, end;
    int i, j;
    
    scanf("%d", &n);
    for (i = 0; i < n; i++){
        scanf("%d%d", &start, &end);
        if (start < end){
            for (j = start; j < end; j++){
                printf("%d ", j);
            }
            printf("%d\n", j);
        }
        else if (start <= 5){
            for (j = start; j > end; j--){
                printf("%d ", j);
            }
            printf("%d\n", j);
        }
        else {
            if (end <= 5){
                j = start;
                while (j != end){
                    printf("%d ", j);
                    if (j == 9){
                        j = 5;
                    }
                    else if (j >= 6){
                        j++;
                    }
                    else {
                        j--;
                    }
                }
                printf("%d\n", j);
            }
            else {
                for (j = start; j <= 9; j++){
                    printf("%d ", j);
                }
                for (j = 5; j >= 0; j--){
                    printf("%d ", j);
                }
                for (j = 1; j < end; j++){
                    printf("%d ", j);
                }
                printf("%d\n", j);
            }
        }
    }
    return (0);
}