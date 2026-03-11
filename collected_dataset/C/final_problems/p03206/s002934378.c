#include<stdio.h>

int main(){
    int D,i;
    scanf("%d",&D);
    printf("Christmas");
    for(i=0;i<25-D;i++){
        printf(" Eve");
    }
    printf("\n");
    return 0;
}