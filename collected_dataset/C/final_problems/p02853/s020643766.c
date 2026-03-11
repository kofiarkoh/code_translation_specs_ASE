#include<stdio.h>
int main() {
 int x, y;
 scanf("%d %d", &x, &y);
 printf("%d", ((x<4)*(4-x)+(y<4)*(4-y)+4*(x*y==1))*100000);
}