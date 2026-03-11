#include<stdio.h>
#include<stdlib.h>
#include<math.h>

int main()
{
    int n,i,ans=0;
    scanf("%d",&n);
  　int p[n];
    for(i=0;i<n;i++)
    {
        scanf("%d",&p[i]);
    }
    for(i=1;i<n-1;i++)
    {
        if(p[i-1]<p[i]&&p[i]<p[i+1]) ans++;
        else if(p[i-1]>p[i]&&p[i]>p[i+1]) ans++;
    }
    printf("%d",ans);
    return 0;
}