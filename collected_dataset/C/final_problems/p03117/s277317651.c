#include <stdio.h>


int main()
{
  int cnt;
  int R, B;
  int M = 0; 
  int i,j,m;
  int K = 0;

  cnt = scanf("%d %d", &R, &B);

  M = R;
  if(M < B) M=B;

  for (m=1; m<M; m++) {
    for (i=0; i<m; i++) {
      for (j=0; j<m; j++) {
        if ((i != (m-1)) && (j != (m-1))) continue;
        if ((R >= i) && (B >= j)) {
          K++;
          R -= i;
          B -= j;
        } else if (B < j) {
          break;
        }
      }
    }
  }

  printf("%d", (K - 1));

  return 0;
} 