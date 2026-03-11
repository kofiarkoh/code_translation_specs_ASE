#include<stdio.h>
int cube(int a){return a*a*a;}
int main(){int n,k;scanf("%d%d",&n,&k);printf("%d",cube(n/k)+(k%2?0:cube(2*n/k-n/k)));}