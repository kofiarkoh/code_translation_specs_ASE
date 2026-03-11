#include<stdio.h>
#include<math.h>

int main()
{
    double a , b ;
    while(scanf("%lf",&a)!=EOF)
    {
       a=a/9.8 ;
       a=4.9*a*a ;
       a=((a+5)/5.0) ;
       a=ceil(a) ;
       printf("%.0lf\n",a) ;
    }
    return 0 ;
}