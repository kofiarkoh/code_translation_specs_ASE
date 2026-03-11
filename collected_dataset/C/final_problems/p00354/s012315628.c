#include<stdio.h>
int main(){
    int x;
    scanf("%d",&x);
    x=x%7;
    if(x==0)printf("thu\n");
    if(x==1)printf("fri\n");
    if(x==2)printf("sat\n");
    if(x==3)printf("sun\n");
    if(x==4)printf("mon\n");
    if(x==5)printf("tue\n");
    if(x==6)printf("wed\n");
    return 0;
}
