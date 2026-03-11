#include <stdio.h>
#include <stdlib.h>


int main (void){
        int K;
        int C;
        scanf("%d",&K);
for (int i = 0; i < K; i++) {
  printf("%d",1+(i%9));
C=i/9;
  for (int j = 0; j < C; j++) {
    printf("9");
  }
  printf("\n");
}


        return 0;
}
