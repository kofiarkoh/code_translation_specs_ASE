#include<stdio.h>

int main()
{
    int N,K;
    int A[1000]={};
    int d;
    int address;

    scanf("%d %d", &N,&K);

    int i, j;
    for(i=0;i<K;i++)
    {
        scanf("%d", &d);
        for(j=0;j<d;j++)
        {
            scanf("%d", &address);
            A[address - 1] = 1;
        }
    }

    int count=0;
    for(i=0;i<N;i++)
    {
        if(A[i]==0)
            count++;
    }

    printf("%d", count);

    return 0;
}