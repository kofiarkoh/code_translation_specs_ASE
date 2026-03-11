#include <stdio.h>
int main(void){
    int b,r,g,c,s,t;
    while(scanf("%d%d%d%d%d%d",&b,&r,&g,&c,&s,&t) , t != 0){
        printf("%d\n",b * 95 + r * 63 + g * 7 + c * 2 + s * 3 - t * 3 + 100);
    }
    return 0;
}