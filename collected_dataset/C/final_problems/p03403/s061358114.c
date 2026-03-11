#include<stdio.h>
#include<stdlib.h>
int main()
{
    int N;
    int A[10050]={0};
    int i,j;
    int sum;
    scanf("%d",&N);
    A[0]=0;
    for(i=1;i<N+1;i++)
    {
        scanf("%d",&A[i]);
    }
    sum=0;
    for(i=1;i<N+1;i++)
    {
        sum+=abs(A[i]-A[i-1]);
    }
    sum+=abs(A[N]-A[0]);
    for(i=1;i<=N;i++)
    {
        printf("%d\n",sum-abs(A[i]-A[i-1])-abs(A[i+1]-A[i])+abs(A[i+1]-A[i-1]));
    }
    return 0;
}
