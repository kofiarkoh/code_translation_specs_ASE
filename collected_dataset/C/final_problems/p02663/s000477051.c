#include<stdio.h>
int main(){
    int a,b,c,d,e,f,g,h;
    scanf("%d %d %d %d %d",&a,&b,&c,&d,&e);
    f=c-1-a;
    g=d+60-b;
    h=f*60+g-e;
    printf("%d",h);
}