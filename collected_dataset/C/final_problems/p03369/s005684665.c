
#include<stdio.h>

int main(void){
    int prime = 700;
    char topping[5];
    scanf("%s", topping);
    for(int i = 0; i < 3; i++){
        if(topping[i] == 'o'){
            prime += 100;
        }
    }
    printf("%d\n", prime);
    return 0;
}