#include <stdio.h>
#include <math.h>

int main(void){
    int a,b,c,d,max;
    scanf("%d %d %d/",&a,&b,&c);
    scanf("%d",&d);
    max=a;
    if(max<=b){
        max=b;
    }
    if(max<=c){
        max=c;
    }
    int e=pow(2,d);
    printf("%d",max*e+a+b+c-max);
}