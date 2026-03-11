#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX(a,b) (a > b) ? a : b
#define MIN(a,b) (a < b) ? a : b
const int inf = 1000000000; // 10^9

int main(){
   int a,b;
   char op;scanf("%d %c %d",&a,&op,&b);
   if(op == '+')printf("%ld",a+b);
   else printf("%d",a-b);
}
