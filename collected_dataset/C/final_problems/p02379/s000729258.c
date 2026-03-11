#include <stdio.h>
#include <math.h>

int main(int argc, const char * argv[]) {
    double x1=0.0,x2=0.0,y1=0.0,y2=0.0;
    double x=0.0,y=0.0,z=0.0;
    scanf("%lf %lf %lf %lf",&x1,&y1,&x2,&y2);
    x = x2-x1;
    y = y2-y1;
    z = x*x + y*y;
    z = sqrt(z);
    
    printf("%.8f\n",z);
    return 0;
}