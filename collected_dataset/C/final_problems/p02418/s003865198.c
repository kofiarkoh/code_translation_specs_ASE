#include <stdio.h>
#include <string.h>
int main(int argc, const char * argv[]) {
    char s[101], p[101], i, ssub[101];
    scanf("%s",s);
    scanf("%s",p);
    for( i = 0 ; s[i] != '\0' ; i++ ) {
        ssub[i] = s[i];
    }
    strcat(s, ssub);
    if( strstr(s, p) != NULL ){
        printf("Yes\n");
    }
    else{
        printf("No\n");
    }
    return 0;
}