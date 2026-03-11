#include <stdio.h>

int main(void) {
  int N, i;
  char List[256];
  char S[128];
  char T[128];
  
  scanf("%d", &N);
  scanf("%s %s", S, T);
  
  for(i=0; i<=N; i++) {
    List[2 * i] = S[i];
    List[2 * i + 1] = T[i];
  }
  
  printf("%s", List);
  return 0;
}