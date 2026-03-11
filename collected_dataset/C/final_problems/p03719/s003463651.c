#include<stdio.h>

int main(){
    int a,b,c;
    scanf("%d%d%d",&a,&b,&c);
    if(a <= c && c <= b){printf("Yes\n");}else{printf("No\n");}
    return 0;
}