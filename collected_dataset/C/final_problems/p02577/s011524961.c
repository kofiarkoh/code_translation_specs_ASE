#include <stdio.h>
#include <math.h>

int main(void){
    char moji[200002];
    scanf("%s",moji);
    int sum=0,i=0;
    while(moji[i]){
        sum+=(int)moji[i]-48;
        i++;
    }
    if(sum%9==0){
        printf("Yes");
    }else{
        printf("No");
    }
}
