#include <stdio.h>

int main(void){
  int i, n, a;
  int max=0, min=1001;

  scanf("%d", &n);

  for(i=0;i<n;i++){
    scanf("%d", &a);
    if(a>max)
    max=a;
    if(a<min)
    min=a;
  }

  printf("%d\n", max-min);
  return 0;
}
