#include <stdio.h>

int main(void){
    int a,b,n;
    scanf("%d %d %d",&a,&b,&n);
    printf("%d\n",(a*n)/b-a*(n/b));
    return 0;
}