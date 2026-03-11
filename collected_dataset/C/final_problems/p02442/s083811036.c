#include<stdio.h>
int main()
{
    int n,m,i,sum1=0,sum2=0;
    scanf("%d",&n);
    int A[n];
    for(i=0;i<n;i++)
    {
        scanf("%d",&A[i]);
    }
        scanf("%d",&m);
        int B[m];
    for(i=0;i<m;i++)
    {
        scanf("%d",&B[i]);
    }
    if(n>m)
    {
        for(i=0;i<m;i++){
            if(B[i]>=A[i])
                sum1=sum1+1;
            else
                sum2=sum2+1;
    }
    if(sum1>sum2)
        printf("1\n");
        else printf("0\n");
    }
    else
    {
        for(i=0;i<n;i++)
        {
            if(B[i]>=A[i])
                sum1=sum1+1;
            else
                sum2=sum2+1;
        }
        if(sum1>sum2)
            printf("1\n");
        else
            printf("0\n");
    }
    return 0;
}

