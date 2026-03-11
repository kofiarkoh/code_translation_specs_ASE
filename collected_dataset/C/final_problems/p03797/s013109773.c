#include <stdio.h>
#define MAX(a,b) ((a>b)?(a):(b))

int main(void){
    int a,b,result;
    scanf("%d%d",&a,&b);
    if(2*a>b){
        result=(b>1)?b/2:0;
    }
    else{
        result=a;
        b-=a*2;
        result+=b/4;
    }
    printf("%d\n",result);
    return 0;
}