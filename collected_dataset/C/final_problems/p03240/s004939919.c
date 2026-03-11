#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
  int N;
  scanf("%d", &N);

  int X[101];
  int Y[101];
  int H[101];
  int F[101][101];
  memset(F, -1, 101*101);

  for(int i = 0; i < N; i++) {
    scanf("%d %d %d", &X[i], &Y[i], &H[i]);
    if(H[i] != 0) {
      for(int x = 0; x <= 100; x++) {
	for(int y = 0; y <= 100; y++) {
	  int tmpH = H[i] + abs(x - X[i]) + abs(y - Y[i]);
	  if(i == 0) {
	    F[x][y] = tmpH;
	  } else {
	    if(tmpH != F[x][y]) {
	      F[x][y] = 0;
	    }
	  }
	}
      }
    }
  }
  
  for(int x = 0; x <= 100; x++) {
    for(int y = 0; y <= 100; y++) {
      if(F[x][y] != 0) {
	printf("%d %d %d\n", x, y, F[x][y]);
	return 0;
      }
    }
  }
  
  return 0;
}
