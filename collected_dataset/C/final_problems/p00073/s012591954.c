#include<stdio.h>
#include<math.h>

int main(void)
{
    int x,h;

    while(scanf("%d%d",&x,&h),x+h) printf("%lf\n",sqrt(x*x+4*h*h)*x+x*x);
    return 0;
}