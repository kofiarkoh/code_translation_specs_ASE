#include <stdio.h>

int main(void){
  int n;
  int i;

  scanf("%d",&n);
  for (i=9;i>0;i--){
    if (n%i==0){
      if (n/i<=9){
        printf("Yes\n");
        return 0;
      }
    }
  }
  printf("No\n");
  return 0;
}
