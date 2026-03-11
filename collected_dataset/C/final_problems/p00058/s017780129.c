#include <stdio.h>

int main(void){
    double x1, x2, y1, y2, x3, x4, y3, y4;
    while(1) {
        if(scanf("%lf %lf %lf %lf %lf %lf %lf %lf", &x1, &y1, &x2, &y2, &x3, &y3, &x4, &y4) != 8) {
            break;
        }
        if(y1 - y2 == 0) {
            if(x3 - x4 == 0) {
                printf("YES\n");
            } else {
                printf("NO\n");
            }
            continue;
        }
        if(y3 - y4 == 0) {
            if(x1 - x2 == 0) {
                printf("YES\n");
            } else {
                printf("NO\n");
            }
            continue;
        }
        double n1, n2;
        n1 = (x1 - x2) / (y1 - y2);
        n2 = (x3 - x4) / (y3 - y4);
        if((x1 - x2) * (x3 - x4) / (y3 - y4) / (y1 - y2) == -1) {
            printf("YES\n");
        } else {
            printf("NO\n");
        }
    }
    return 0;
}