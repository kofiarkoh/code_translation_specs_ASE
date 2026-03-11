#include<stdio.h>
#include<string.h>
int main ()
{
    char A[10],B[10],C[10];
    scanf("%s %s %s",&A,&B,&C);
    if((A[strlen(A)-1]==B[0]) && (B[strlen(B)-1]==C[0]) )
        printf("YES");
    else printf("NO");
    return 0;
}
