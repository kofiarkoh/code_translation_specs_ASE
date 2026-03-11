#include<stdio.h>
#include<math.h>
int main(){
    double r,pi;
    scanf("%lf",&r);
    pi = acos(-1);
    printf("%lf %lf\n",pi*r*r,2*pi*r);
    
    return 0;
}

