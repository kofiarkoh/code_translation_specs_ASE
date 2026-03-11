#include<stdio.h>

int main(void){
    char x[1003];
    int sum,i;

    while(1){
        scanf("%s",x);
        if(x[0] ==  '0' && x[1] == '\0')
            break;
        sum = 0;i = 0;
        while(x[i] != '\0')
            sum += x[i++] - '0';
        printf("%d\n",sum);
    }
    return 0;
}

