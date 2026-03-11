#include<stdio.h>
int main()
{
    int n,a,b;
    
    scanf("%d %d %d",&n ,&a,&b);
    
    int p=a;
    if(a>b)p=b;
    
    int q=a+b-n;
    if(q<0)q=0;
    
    printf("%d %d\n",p,q);
    
    return 0;
}