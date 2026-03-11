#include <stdio.h>

int main()
{
  int i, N, ret;
  char S[100001];
  
  ret = scanf("%d", &N);
  ret = scanf("%s", S);
  ret = N;
  for ( i=0; i<N-1; i++ ) {
    if ( S[i] == S[i+1] )
      ret--;
  }
  printf("%d\n", ret);
}

  