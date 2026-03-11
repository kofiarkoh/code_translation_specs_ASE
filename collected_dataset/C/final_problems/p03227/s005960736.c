#include<stdio.h>

int main(){
    char s[3] = {'\0','\0','\0'};
    scanf("%s",s);
    if(s[2] != '\0'){
        char a = s[0];
        s[0] = s[2];
        s[2] = a; 
    }
    printf("%s\n",s);
    return 0;
}

