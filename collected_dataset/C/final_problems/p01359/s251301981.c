#include <stdio.h>

char era_name[1000][20];
int era_year[1000];
int western[1000];
int year;
int i, j;

int main(void)
{
    int N, Q;
    
    while (1){
        scanf("%d%d", &N, &Q);
        if (N == 0 && Q == 0){
            break;
        }
        
        for (i = 0; i < N; i++){
            scanf("%s%d%d", era_name[i], &era_year[i], &western[i]);
        }
        
        for (i = 0; i < Q; i++){
            scanf("%d", &year);
            for (j = 0; j <N ; j++){
                if (western[j] - era_year[j] + 1 <= year && year <= western[j]){
                    printf("%s %d\n", era_name[j],
                                      year - (western[j] - era_year[j] + 1) + 1);
                    break;
                }
            }
            if (j == N){
                printf("Unknown\n");
            }
        }
    }
    
    return (0);
}