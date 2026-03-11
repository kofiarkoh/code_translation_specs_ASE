#include <stdio.h>

int main(void){
    int a, b;
    scanf("%d %d", &a, &b);

    int digit_b = 0, copy_b = b;
    while(copy_b > 0){
        copy_b /= 10;
        digit_b++;
    }

    for(int i = 0; i < digit_b; i++){
        a *= 10;
    }
    a += b;

    int is_sq = 0;
    for(int i = 1; i * i <= a; i++){
        if(i * i == a){
            is_sq = 1;
            break;
        }
    }

    if(is_sq) printf("Yes\n");
    else printf("No\n");
    return 0;
}