#include <stdio.h>
#include <string.h>

int main(void)
{
    char A[1001];
    char B[1001];
    int len_a, len_b;
    int i, j;
    int flag;
    
    scanf("%s%s", A, B);
    
    len_a = strlen(A);
    len_b = strlen(B);
    
    flag = 0;
    for (i = 0; i < len_a - len_b + 1; i++){
        for (j = 0; j < len_b; j++){
            if (B[j] == '_'){
                continue;
            }
            if (B[j] != A[i + j]){
                break;
            }
        }
        if (j == len_b){
            flag = 1;
            break;
        }
    }
    
    if (flag == 0){
        printf("No\n");
    }
    else {
        printf("Yes\n");
    }
    
    return (0);
}

