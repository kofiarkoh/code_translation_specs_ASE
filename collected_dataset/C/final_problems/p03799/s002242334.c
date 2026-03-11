#include<stdio.h>
int main()
{
    long int n,m,count;
    while(scanf("%ld%ld",&n,&m)!=EOF)
    {
        count=(n*2+m)/4;
        printf("%ld\n",count);
    }
    return 0;
}
