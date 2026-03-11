#include<stdio.h>
#include<string.h>

int main(void){
    int i;
    char s[10000];
    scanf("%s",s);
    for(i=0;i<strlen(s)-8;i++){
        printf("%c",s[i]);
    }
    return 0;
}