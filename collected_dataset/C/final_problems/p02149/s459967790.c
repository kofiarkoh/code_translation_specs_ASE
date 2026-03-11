#include <stdio.h>

int a, b, c;
int main(void){
    scanf("%d %d %d", &a, &b, &c);
    if(a>b&&a>c){
        printf("A\n");
    }else if (b>c){
        printf("B\n");
    }else{
        printf("C\n");
    }
    return 0;
}

