#include <stdio.h>
#include <math.h>

#define EPS (1e-5)

double getAreaTriangle(int theta)
{
    return (0.25 * sqrt(2 +  2 * cos(theta * M_PI / 180)) * sqrt(2 - 2 * cos(theta * M_PI / 180)));
}

int main(void)
{
    int i;
    int m, n;
    double area[2];
    int sum, theta;
    
    while (1){
        scanf("%d", &m);
        
        if (m == 0){
            break;
        }
        area[0] = area[1] = 0;
        
        sum = 360;
        for (i = 0; i < m - 1; i++){
            scanf("%d", &theta);
            area[0] += getAreaTriangle(theta);
            sum -= theta;
        }
        area[0] += getAreaTriangle(sum);
        
        scanf("%d", &n);
        
        sum = 360;
        for (i = 0; i < n - 1; i++){
            scanf("%d", &theta);
            area[1] += getAreaTriangle(theta);
            sum -= theta;
        }
        area[1] += getAreaTriangle(sum);
        
        if (fabs(area[0] - area[1]) < EPS){
            printf("0\n");
        }
        else if ((area[0] - area[1]) > EPS){
            printf("1\n");
        }
        else {
            printf("2\n");
        }
    }
    
    return (0);
}