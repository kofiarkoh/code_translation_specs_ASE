#include <stdio.h>
#include <string.h>

int main(){
    int N;
    int a,b;
    scanf("%d",&N);
    
    for(a = 1; a <= 9; a++) for(b = 1; b <= 9; b++)
        if(a * b == N){
            printf("Yes");
            return 0;
        }
    printf("No");
}