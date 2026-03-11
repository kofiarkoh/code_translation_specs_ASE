#include <stdio.h>

int main(void)
{
    int n;
    int h, m;
    int i;
    double angle;
    
    scanf("%d", &n);
    for (i = 0; i < n; i++){
        scanf("%d:%d", &h, &m);
        
        angle = h * 360 / 12 + 360 / 12 * m / 60.0;
        angle -= (360 / 60 * m);
        
        if (angle > 180){
            angle -= 360;
        }
        
        if (-30 < angle && angle < 30){
            printf("alert\n");
        }
        else if (-90 < angle && angle < 90){
            printf("warning\n");
        }
        else {
            printf("safe\n");
        }
    }
    return (0);
}