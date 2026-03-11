
#include<stdio.h>

int isPower(int x){
    int k = 0, i;
    for(i = x; i > 1; i--){
        for(int j = 2; j < i; j++){
            k = j;
            while(k <= i){
                if(k == i){
                    return i;
                }
                k *= j;
            }
        }
    }
    return i;
}

int main(void){
    int x;
    scanf("%d", &x);
    printf("%d\n", isPower(x));
    return 0;
}