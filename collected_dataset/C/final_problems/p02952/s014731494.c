#include<stdio.h>

int main(void){
    int n;
    scanf("%d",&n);

    int count = 0;

    for(int i=1; i<=n; i++){
        if(i<10){
            count++;
        }else if(99<i && i<1000){
            count++;
        }else if(9999<i && i<100000){
            count++;
        }
    }

    printf("%d",count);

    return 0;
}