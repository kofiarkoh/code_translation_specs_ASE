#include<stdio.h>
 
char n;


int main(){
    scanf("%c",&n);

    if((n>='A') && (n<='Z')){
        printf("A\n");
    }
    else{
        printf("a\n");
    }   
    return 0;
}
