#include <stdio.h>
#include <math.h>

int main()
{
    int N,M;

    scanf("%d %d",&N,&M);

    printf("%d\n",(int)((1900*M+100*(N-M))*pow(2,M)));

    return 0;
}