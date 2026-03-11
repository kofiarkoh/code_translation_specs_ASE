#include <stdio.h>
#include <string.h>

int main(){
    int i = 0;
    char s[10001];
    char c[2];

    scanf("%s", s);

    while(i < 26){
        c[0] = 'a' + i;
        if(strstr(s, c) == NULL){
            printf("%c\n", c[0]);
            return 0;
        }
        i++;
    }
    printf("None\n");

    return 0;
}