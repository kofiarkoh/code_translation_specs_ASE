#include<stdio.h>

int main(void)
{
    int N,M,P,i;
    int X[1000];
    int sum;

    while(scanf("%d%d%d",&N,&M,&P),N+M+P){

        sum=0;
        for(i=1;i<=N;i++){
            scanf("%d",&X[i]);
            sum+=X[i];
        }

        printf("%d\n",X[M]!=0?(100-P)*sum/X[M]:0);
    }
    return 0;
}