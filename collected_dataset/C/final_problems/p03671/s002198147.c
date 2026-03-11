#include <stdio.h>
int main(void){
    int a,b,c,tmp = 0;
    scanf("%d %d %d",&a,&b,&c);
    if(a > b){
        tmp = a;
        a = b;
        b = tmp;
    }
    if(b > c){
        tmp = b;
        b = c;
        c = tmp;
    }
    printf("%d",a + b);

}
