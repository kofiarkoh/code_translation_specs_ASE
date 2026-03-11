#include<stdio.h>
#include<stdlib.h>
int fun(int ,int ,int );
int fun(int x,int y,int z)
{
    int ans=0;
    while(ans*y+(ans+1)*z<=x)
    {
        ans++;
    }
    return ans-1;
}
int main()
{
    int x,y,z;
    scanf("%d%d%d",&x,&y,&z);
    printf("%d\n",fun(x,y,z));
    return 0;
}