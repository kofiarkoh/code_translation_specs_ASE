#include <stdio.h>
int main(void){
    int a,b,c;
    scanf("%d%d%d", &a, &b, &c);
    if(a+b==c) puts("Yes");
    else if(a+c==b) puts("Yes");
    else if(b+c==a) puts("Yes");
    else puts("No");
    return 0;
}