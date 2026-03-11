#include <stdio.h>

int main(void){
    int a,cnt;
    scanf("%d",&a);
    cnt = 0;
    cnt+=a/25;
    a = a%25;
    cnt += a/10;
    a = a%10;
    cnt += a/5;
    a = a%5;
    cnt += a;
    printf("%d\n",cnt);
}

