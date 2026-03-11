#include<stdio.h>

int main()
{
    int n,i,x,t=1;
    scanf("%d",&n);

    for(i=0;i<n;i++)
    {
        scanf("%d",&x);
        if(x%2==0) if(x%3 && x%5) t=0;
    }

    if(t) printf("APPROVED\n");
    else printf("DENIED\n");
}
