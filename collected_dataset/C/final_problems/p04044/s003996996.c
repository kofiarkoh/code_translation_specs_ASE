#include <stdio.h>
int main(void){
  int i, N, L;
  char s1[100][101], b;
  scanf("%d %d", &N, &L);
  for(int i=0; i<N; i++){
    scanf("%s", s1[i]);
  }
  i=0;
  for(int j = 0; j < L; j++){
  	while(i < N-1){
      if((s1[i][j] > s1[i+1][j] && j == 0)|| (s1[i][j] > s1[i+1][j] && s1[i][j-1] == s1[i+1][j-1])){
        b = s1[i][j];
        s1[i][j] = s1[i+1][j];
        s1[i+1][j] = b;
        i = 0;
      }
      i++;
    }
  }
  for(i = 0; i<N; i++){
    printf("%s", s1[i]);
  }
  return 0;
}