// AOJ Vol3-0380 Celsius and Fahrenheit
// 2018.12.22

//#define _CRT_SECURE_NO_WARNINGS

#include <stdio.h>

int main(){
    int F, C;
    
    scanf("%d", &F);
    C = (F - 30)/ 2;
    printf("%d\n", C);
    return 0;
}
