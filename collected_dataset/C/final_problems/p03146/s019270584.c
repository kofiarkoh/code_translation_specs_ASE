#include<stdio.h>

int main(void){
    int num,i;
    i = 1;
    scanf("%d",&num);
    
    while(1){
        if(num == 4 || num == 1){
            break;
        }
        
        i += 1;
        if(num%2==0){
            num = num/2;
        }else{
            num = num * 3 + 1;
        }
    }
    printf("%d",i+3);
    
    return 0;
}