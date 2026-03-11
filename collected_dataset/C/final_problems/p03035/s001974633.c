#include<stdio.h>
int main(void){
    int a,b;
    scanf("%d %d",&a,&b);
    if(a>12){
        printf("%d",b);
    }else if(a>5){
        printf("%d",b/2);
    }else{
        printf("%d",0);
    }
}