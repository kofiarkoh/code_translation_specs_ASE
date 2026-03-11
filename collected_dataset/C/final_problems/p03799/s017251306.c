#include<stdio.h>
int main(void)
{
    double a,b,sum=0;
    int c;
    scanf("%lf%lf",&a,&b);
    if(a>=b/2)
    {
        sum=b/2;
    }
    else if(a<b/2)
    {
        sum=a+(b-2*a)/4;
    }
    c=sum;
    if(sum-c>=0.5)
        sum=sum-1;
    printf("%.0f",sum);
    return 0;
}