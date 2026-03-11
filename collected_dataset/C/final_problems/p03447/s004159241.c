#include <stdio.h>

int main(void){
    int a, b, x;
    scanf("%d%d%d", &x, &a, &b);
    printf("%d\n", (x-a) % b);
    return 0; 
}