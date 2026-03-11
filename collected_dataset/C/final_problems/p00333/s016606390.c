#include<stdio.h>

int func(int a,int b)
{
    int r;
    r = a % b;
　　while (r > 0)
　　　　{
　　　　　　a = b;
　　　　　　b = r;
　　　　　　r = a % b;
　　　　}
　　return b;
}
int main()
{
    int w,h,c;
    int s;
    
    scanf("%d %d %d",&w,&h,&c);
    s = func( w,h );
    
    h /= s;
    w /= s;
    
    printf("%d\n",h*w*c);
    
    return 0;
}
