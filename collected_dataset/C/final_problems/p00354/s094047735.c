#include <stdio.h>
int main(void){
    int x;
    scanf("%d",&x);
    if(x%7==0){
        printf("thu\n");
    }if(x%7==1){
        printf("fri\n");
    }if(x%7==2){
        printf("sat\n");
    }if(x%7==3){
        printf("sun\n");
    }if(x%7==4){
        printf("mon\n");
    }if(x%7==5){
        printf("tue\n");
    }if(x%7==6){
        printf("wed\n");
    }
    return 0;
}

