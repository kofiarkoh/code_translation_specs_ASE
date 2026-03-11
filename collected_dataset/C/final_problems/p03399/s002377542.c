#include <stdio.h>

int main(void){
  int a[2],b[2],m[2];

  scanf("%d %d %d %d",&a[0],&a[1],&b[0],&b[1]);
  if(a[0]> a[1]){
    m[0] = a[1];
  }else{
    m[0] = a[0];
  }
  if(b[0]> b[1]){
    m[1] = b[1];
  }else{
    m[1] = b[0];
  }
  printf("%d\n",m[0]+m[1]);
  return 0;
}