#include<stdio.h>
int main(void)
{
    int n,i,count=0,ans=0;
    scanf("%d",&n);
    int h[n];
    for(i=0;i<n;i++)
    {
        scanf("%d",&h[i]);
    }
    for(i=0;i<n-1;i++)
    {
        if(h[i]>=h[i+1])
        {
            count++;
            if(ans<count){
                ans=count;
                }
        }
        else
        {
        count = 0;
        }

    }
    printf("%d\n",ans);
    return 0;
}
