#include <stdio.h>
#include <string.h>
int main(void){
    char s[100];
    scanf("%s", s);

    int number = strlen(s) - 2;
    printf("%c%d%c\n", s[0], number, s[strlen(s) - 1]);

    return 0;
}
