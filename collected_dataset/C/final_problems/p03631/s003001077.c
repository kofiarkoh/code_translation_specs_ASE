#include<stdio.h>
#include<stdlib.h>
#include<math.h>
#include<string.h>

/*
各要素は親の値を持っておけばいい

*/

int main(void){
    char s[4];
    scanf("%s",s);
    if(s[0] == s[2]){
        printf("Yes\n");
    }else{
        printf("No\n");
    }
    return 0;
}