#include<stdio.h>
int main()
{
    int N,flag=0,count=0;
    static int b[200000];
    scanf("%d",&N);
    for(int i=0;i<N;i++)
        scanf("%d",&b[i]);
    for(int i=0;i<N;i++)
    {
        for(int j=0;j<=i;j++)
            if(b[i]<=b[j])
                flag=1;
            else
            {
                flag=0;
                break;
            }
        if(flag)
        count++;
    }
    printf("%d",count);
    return 0;
}
