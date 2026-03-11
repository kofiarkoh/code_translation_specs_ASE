#include <stdio.h>
#include <string.h>
int main(void){
    char a[100], b[100];
    scanf("%s%s", a, b);
    int a_keta = strlen(a), b_keta=strlen(b);
    if(a_keta>b_keta){
        puts("GREATER");
        return 0;
    }
    else if(b_keta>a_keta){
        puts("LESS");
        return 0;
    }
    for(int i=0; a[i]||b[i]!='\0'; i++){
        if(a[i]>b[i]){
            puts("GREATER");
            return 0;
        }
        else if(b[i]>a[i]){
            puts("LESS");
            return 0;
        }
    }
    puts("EQUAL");
    return 0;
}