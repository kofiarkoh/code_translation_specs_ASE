#include<stdio.h>
int main(){
    int n;
    scanf("%d",&n);
    int cnt=0;
    if(n>24){
    while(n>24){
        n=n-25;
        cnt++;
    }
    }
    if(n>9){
    while(n>9){
        n=n-10;
        cnt++;
    }
    }
    if(n>4){
    while(n>4){
        n=n-5;
        cnt++;
    }
    }
    cnt=cnt+n;
    printf("%d\n",cnt);
}
