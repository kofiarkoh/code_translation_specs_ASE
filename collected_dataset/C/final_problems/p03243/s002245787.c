#include<stdio.h>
int main()
{
    int n,n1[3],i=3,r;
    scanf("%d",&n);
    while(n>0 && i>=0)
    {
        i--;
        r=n%10;
        n1[i]=r;
        n=n/10;
    }
    for(i=0;i<2;i++)
    {
        for(r=(i+1);r<3;r++)
        {
            if(n1[i]<n1[r])
            {
                n=n1[r];
                n1[r]=n1[i];
                n1[i]=n;
            }
        }
    }
    n=(n1[0]*100)+(n1[0]*10)+n1[0];
    printf("%d",n);
    return 0;
}