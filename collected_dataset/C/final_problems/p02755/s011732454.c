#include <stdio.h>

int main(){
  int a, b;  scanf("%d%d", &a, &b);

  for(int i=13; i<103; i++){
    if((int)(i*0.08) == a && (int)(i*0.1) == b){
      printf("%d\n", i);
      return 0;
    }
  }
  
  printf("-1\n");
  return 0;
}
