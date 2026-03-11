#include <stdio.h>

int main(void){
    char s[20];

    scanf("%s", s);

    printf("2018/01/%c%c\n", s[8], s[9]);

    return 0;
}