#include<stdio.h>
int main(){
    int x,a,b;
    int d1,d2;
    scanf("%d %d %d",&x,&a,&b);
    d1=x-a;
    d2=x-b;
    if(d1<0){
        d1*=-1;
    }
    if(d2<0){
        d2*=-1;
    }
    if(d1>d2){
        printf("B");
    }else{
        printf("A");
    }
    return 0;
}