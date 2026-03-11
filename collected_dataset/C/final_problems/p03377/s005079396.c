#include <stdio.h>
int main(){
  int a,b,x;
  scanf("%d%d%d",&a,&b,&x);
  (b>=x-a&&x-a>=0)?puts("YES"):puts("NO");
  return 0;
}