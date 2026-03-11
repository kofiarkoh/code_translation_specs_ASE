#include <stdio.h>
#include <string.h>

int main()
{
    int A, B;
    char S[20], T[20], U[20];
    scanf("%s %s", S, T);
    scanf("%d %d", &A ,&B);
    scanf("%s",U);
    if((strcmp(U,S)) == 0){
    	A--;
    }
    else{
    	B--;
    }
    printf("%d %d", A, B);
    return 0;
}