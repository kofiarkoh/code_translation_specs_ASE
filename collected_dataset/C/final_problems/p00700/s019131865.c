#include <stdio.h>

int main(void)
{
    int N, i;
    scanf("%d", &N);
    for(i = 0; i < N; i++){
        int x, y, dx, dy;
        int max_d, max_x, max_y;
        x = y = max_d = max_x = max_y = 0;
        while(1){
            scanf("%d %d", &dx, &dy);
            if(dx == 0 && dy == 0)
                break;
            x += dx;
            y += dy;
            if(max_d < (x * x) + (y * y)){
                max_d = (x * x) + (y * y);
                max_x = x;
                max_y = y;
            }else if(max_d == (x * x) + (y * y) && max_x < x){
                max_d = (x * x) + (y * y);
                max_x = x;
                max_y = y;
            }
        }
        printf("%d %d\n", max_x, max_y);

    }
    return 0;
}