#include <stdio.h>

int main(void){
  int N;
  char A[101];
  char B[101];
  char C[101];

  scanf("%d", &N);
  scanf("%s", A);
  scanf("%s", B);
  scanf("%s", C);

  int change = 0;
  for(int i = 0; i < N; i++) {
    if(A[i] == B[i]) {
      if(B[i] == C[i]) {// A==B==C
	;
      } else {// A==B , C
	change++;
      }
    } else {// A!=B
      if(B[i] == C[i]) {// A!=B, B==C
	change++;
      } else {// A!=B , B!=C
	if(A[i] == C[i]) {// A==C, B
	  change++;
	} else {
	  change+=2;
	}
      }
    }
  }

  printf("%d\n", change);
  return 0;
}
