#include<stdio.h>
#include<stdlib.h>
#include<limits.h>
#include<string.h>

int main(){
    int A,B,C;
    scanf("%d %d %d",&A,&B,&C);
    if(B/A > C) printf("%d",C);
    else printf("%d",B/A);
    return 0;   
}