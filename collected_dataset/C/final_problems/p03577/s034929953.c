#include <stdio.h>
#include <string.h>
int main (void){
    char s[100] = {0};
    int i;
    int n;
    scanf("%s",s);
    n = strlen(s);
    for(i=0;i<n-8;i++){
        printf("%c",s[i]);
    }
    printf("\n");
    
    return 0;
}