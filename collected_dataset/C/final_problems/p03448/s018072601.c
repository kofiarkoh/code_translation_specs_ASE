#include <stdio.h>
#include <stdlib.h>

int main()
{
    int i, j, k, s, X, A, B, C;
    s = 0;
    scanf("%d",&A);
    scanf("%d",&B);
    scanf("%d",&C);
    scanf("%d",&X);
    for(i=0;i<=A;i++)
        for(j=0;j<=B;j++)
            for(k=0;k<=C;k++)
               if(i*500+j*100+k*50==X)
                  s++;
    printf("%d",s);
    return 0;
}