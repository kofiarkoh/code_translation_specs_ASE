#include <stdio.h>

int main(){
  char pat[4][9][9];
  int i, j, k;
  for(i = 0; i < 8; ++i) scanf("%s", pat[0][i]);
  for(i = 1; i < 4; ++i){
    printf("%d\n", i * 90);
    for(j = 0; j < 8; ++j)
      for(k = 0; k < 8; ++k)
        pat[i][k][7 - j] = pat[i - 1][j][k];

    for(j = 0; j < 8; ++j){
      for(k = 0; k < 8; ++k) printf("%c", pat[i][j][k]);
      puts("");
    }
  }
  return 0;
}