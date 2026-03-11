#include <stdio.h>
#include <stdlib.h>

const long int M = 998244353; // 64-bits

int main(void)
{
  int N;
  int* A;
  int i, s, len, k;
  int *lengths, *heads, *tails;
  int tail;
  int killed;
  long int result; // 64-bits
  
  scanf("%d",&N);
  A = malloc(sizeof(int)*N);
  for(i=0; i<N; i++) {
    scanf("%d",A+i);
    A[i]--;
  }

  s=0;
  if(A[0]==A[N-1]) {
    while(s<=N-1 && A[s]==A[0]) {
      s++;
    }
  }

  if(s==N) {
    if(A[0]==0) {
      result = 1;
      for(i=1;i<=N;i++) {
	result = (result * i) % M;
      }
    } else {
      result = 0;
    }
    printf("%ld\n",result);
    return(0);
  }

  lengths = calloc(N,sizeof(int));
  heads = malloc(sizeof(int)*N);
  tails = malloc(sizeof(int)*N);

  if(s > 0) {
    tail = A[0];
  } else {
    tail = A[N-1];
  }

  len= 1;
  for(i=s; i<N; i++) {
    if(i<N-1 && A[i+1]==A[i]) {
      len++;
    } else {
      if(i<N-1) {
	if(lengths[A[i+1]]>0) {
	  printf("%d\n",0);
	  return(0);
	} else {
	  heads[A[i]] = A[i+1];
	}
      } else {
	heads[A[i]] = A[0];
      }

      lengths[A[i]] = len;
      len = 1;
      tails[A[i]] = tail;
      tail = A[i];
    }
  }

  if(s > 0) {
    heads[A[0]] = A[s];
    lengths[A[0]] += s;
  }

  k = lengths[0];
  if(k == 0) {
    printf("%d\n",0);
    return(0);
  }

  result = 1;
  killed = 0;
  for(i=1; i<N; i++) {
    if(lengths[i] == 0) {
      result = (result * killed) % M;
      killed--;
    } else if(lengths[i] > k) {
      printf("%d\n",0);
      return(0);
    } else if(heads[i] > i && tails[i] > i) {
      if(lengths[i] != k) {
	printf("%d\n",0);
	return(0);
      }
    } else if(heads[i] < i && tails[i] < i) {
      result = (result * (k-lengths[i]+1)) % M;
      killed += (lengths[i]+k-2);
    } else {
      // head < i && tail > i
      // head > i && tail < i
      killed += (lengths[i]-1);
    }

    if(result == 0) {
      break;
    }
  }
  
  printf("%ld\n",result);

  return(0);
}