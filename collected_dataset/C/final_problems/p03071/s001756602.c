#include <stdio.h>

int main(void) {
  int a,b;
  int total=0;
  scanf("%d%d",&a,&b);
  for(int i=1;i<=2;i++) {
    if(a>=b) {
      total+=a;
      a--;
    }else{
      total+=b;
      b--;
    }
  }
  printf("%d\n",total);
  return 0;
}