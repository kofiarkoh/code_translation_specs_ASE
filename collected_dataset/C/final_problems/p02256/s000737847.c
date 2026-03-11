#include<stdio.h>

void cal(int a, int b){
  int c, i;
  c = a%b;
  for (i=c; i>0; i--) {
    if (c%i==0 && b%i==0) {
      printf("%d\n", i);
      break;
    }
  }
}

int main(){
  int x, y;
  scanf("%d %d", &x, &y);
  if (x>y) {
    cal(x, y);
  } else if (y>x) {
    cal(y, x);
  } else {
    printf("%d\n", x);
  }
  return 0;
}