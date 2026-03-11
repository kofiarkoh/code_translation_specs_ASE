#include<stdio.h>
int main(void){
    int x,y,a =0;
    scanf("%d %d",&x,&y);
    
    switch(x){
        case 1 : a+= 300000; break;
        case 2 : a+= 200000; break;
        case 3 : a+= 100000; break;
        default : break;
    }
    switch(y){
        case 1 : a+= 300000; break;
        case 2 : a+= 200000; break;
        case 3 : a+= 100000; break;
        default : break;
    }
    if(x==1&&y==1){
        printf("%d",a+400000);
        }
    else    printf("%d",a);
}