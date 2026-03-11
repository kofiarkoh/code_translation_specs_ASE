#include <stdio.h>
int main(void){
   int i, n, x1, y1, x2, y2, x3, y3, x4, y4, l0, l1, l2, l3;
   scanf("%d", &n);
for (i = 0; i < n; i++){
    scanf("%d %d %d %d %d %d %d %d", &x1, &y1, &x2, &y2, &x3, &y3, &x4, &y4);
    if(x1 == x2 && x2 == x3 && x3 == x4){
        l0 = (y1 - y3) * (y2 - y4);
        l1 = (y1 - y4) * (y2 - y3);
         if(l0 > 0 && l1 > 0){
            printf("0\n");
         }else{
            printf("1\n");
         }
    }else if(y1 == y2 && y2 == y3 && y3 == y4){
        l0 = (x1 - x3) * (x2 - x4);
        l1 = (x1 - x4) * (x2 - x3);
         if(l0 > 0 && l1 > 0){
            printf("0\n");
         }else{
            printf("1\n");
         }
    }else{
    l0 = (x1 - x2)*(y3 - y1) + (y1 - y2)*(x1 - x3);
    l1 = (x1 - x2)*(y4 - y1) + (y1 - y2)*(x1 - x4);
    l2 = (x3 - x4)*(y1 - y3) + (y3 - y4)*(x3 - x1);
    l3 = (x3 - x4)*(y2 - y3) + (y3 - y4)*(x3 - x2);
    if(l0 * l1 <= 0 && l2 * l3 <= 0){
        printf("1\n");
    }else{
        printf("0\n");
    }
    }
}

    return 0;
}

