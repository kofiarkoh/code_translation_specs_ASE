#include <stdio.h>
#include <math.h>

int main() {
    double mov, dir;
    double x, y, d;
    
    while (scanf("%lf,%lf",&mov,&dir),mov != 0 || dir != 0) {
        x += sin(d*M_PI/180)*mov;
        y += cos(d*M_PI/180)*mov;
        
        d += dir;
    }
    
    printf("%d\n%d\n", (int)x, (int)y);
    
    return 0;
}