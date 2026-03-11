#include <stdio.h>

int main()
{
  int N;
  char S[10001];
  int syu[10001];
  int i;

  scanf("%d", &N);
  scanf("%s", &S);

  //  1 = S
  // -1 = W
  syu[0] = 1;
  syu[1] = 1;
  
  for(i = 1; i < N; i++) {
    if(syu[i] == 1) {
      if(S[i] == 'o') {
	syu[i+1] = syu[i-1];
      } else { //S[i] == 'x'
	syu[i+1] = -syu[i-1];
      }
    } else { //syu[i] == -1
      if(S[i] == 'o') {
	syu[i+1] = -syu[i-1];
      } else { //S[i] == 'x'
	syu[i+1] = syu[i-1];
      }
    }
  }

  if(syu[0] != syu[N]) {
    printf("%d\n", -1);
  } else {
    for(i = 0; i < N; i++) {
      if(syu[i] == 1) {
	printf("S");
      } else {
	printf("W");
      }
    }
    printf("\n");
  }
  
  return 0;
}
