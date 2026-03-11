#include<stdio.h>

int main() {
    int h1,m1,h2,m2,k;
    scanf("%d %d %d %d %d",&h1,&m1,&h2,&m2,&k);
    int r=60*h2+m2-(60*h1+m1)-k;
    printf("%d",r);
    return 0;
}