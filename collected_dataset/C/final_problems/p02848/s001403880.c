#include <stdio.h>
#include <string.h>
int main(void){
    int n,i;
    scanf("%d",&n);
    char s[10001];
    scanf("%s",s);
    for(i=0;i<strlen(s);i++){
        printf("%c",(s[i]-'A'+n)%26+'A');
    }
    return 0;
}