#include<stdio.h>
#include<stdlib.h>

int main(void){
  int n;
  char A[100];
  int S[100];
  int a,v;
  int p=0;

  while( scanf("%s", A) != EOF){ 
    // printf("%s\n",A);
    a = atoi(A);
    if(a!=0){
      S[p] = a;
      // printf("S[%d] = %d\n",p,S[p]);
      p++;
    }else{
      if(A[0]=='+')
	S[p-2] = S[p-2] + S[p-1];
      if(A[0]=='-')
	S[p-2] = S[p-2] - S[p-1];
      if(A[0]=='*')
	S[p-2] = S[p-2] * S[p-1];
      // printf("S[%d] = %d\n",p-2,S[p-2]);
      p--;
    }
  }
  printf("%d\n",S[0]);
  return 0;
}