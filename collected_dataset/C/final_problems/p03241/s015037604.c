#include <stdio.h>

int main(){
  int N, M;
  scanf("%d", &N);
  scanf("%d", &M);

  for(int i = (int)((double)M/(double)N)+2; i > 0; i--){
    if (M%i == 0){
      printf("%d\n",i);
      break;
    }
  }
  return 0;
}
