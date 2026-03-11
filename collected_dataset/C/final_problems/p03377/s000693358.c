#include<stdio.h>


int main(void) {
  int i,a,b,catNumber;

  scanf("%d %d %d",&a,&b,&catNumber);


  for(i = b; i >= 0 ; i--) {
    if(a == catNumber) {
      printf("YES\n");
      return 0;
    }
    a += 1;
  }

  printf("NO\n");
  return 0;
}