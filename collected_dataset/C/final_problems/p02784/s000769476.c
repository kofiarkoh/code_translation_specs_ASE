#include <stdio.h>
#include <stdlib.h>

int main()
{
    int H,N,i,sum=0;
    scanf("%d %d",&H,&N);
    int A[N];
    for (i=0;i<N;i++)
    {
        scanf("%d",&A[i]);
        sum+=A[i];
    }
    if (sum>=H)
        printf("Yes");
    else
        printf("No");
    return 0;
}
