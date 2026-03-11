#include <stdio.h>
int main(void){
    char s[200];
    int i;
    int co = 0,all = 0;
    scanf("%s", s);
    for(i = 0;s[i] != '\0';i ++){
        all ++;
    }
    for(i = 0;i < all / 2;i ++){
        if(s[i] != s[all - (i + 1)]){
            co ++;
        }
    }
    printf("%d\n", co);
    return 0;
}