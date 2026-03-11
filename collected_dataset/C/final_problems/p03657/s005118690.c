#include <stdio.h>
int main(void){
    int a,b;
    scanf("%d %d",&a,&b);
    if(a%3==0||b%3==0||(a+b)%3==0){
        printf("Possible");
    }else{
        printf("Impossible");
    };
    return 0;
};
