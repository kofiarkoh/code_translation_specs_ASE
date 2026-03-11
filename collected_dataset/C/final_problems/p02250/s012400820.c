#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N_T 1000001
#define N_P 1001
#define BASE 64

int* kmpSearch(char*, char*);
int* initNext(char*);

int myIndex(char);

int main() {
  char T[N_T];
  char P[N_P];
  int* pos;

  int i;
  int Q;
    
  scanf("%s", T);
  scanf("%d", &Q);
  for (i = 0; i < Q; ++i) {
    scanf("%s", P);
    pos = kmpSearch(T, P);
    printf("%d\n", pos[0]?1:0);
    free(pos);
  }

  return 0;
}

int* kmpSearch(char* T, char* P) {
  int i, j;
  int cnt = 0;
  int M   = strlen(P);
  int N   = strlen(T);
  
  int* next;
  int* pos;

  if (N < M) {
    pos    = (int*)malloc(sizeof(int)*1);
    pos[0] = 0;
    return pos;
  } else {
    pos = (int*)malloc(sizeof(int)*(N-M+2));
  }
  
  next = initNext(P);  
  for (i = 0, j = 0; i < N; ++i, ++j) {
    while ((j >= 0) && (T[i] != P[j])){
      j = next[j];
    }

    if (j == M - 1) {
      cnt++;
      pos[cnt] = i - M + 1;

      do {
	j = next[j];
      }	while ((j >= 0) && (T[i] != P[j]));
    }
  }
  pos[0] = cnt;

  free(next);
  
  return pos;
}

int* initNext(char* P){
  int i, j;
  int M = strlen(P);
  int* next = (int*)malloc(sizeof(int)*M);

  next[0] = -1;
  for (i = 0, j = -1; i < M; ++i, ++j, next[i] = j) {
    while ((j >= 0) && (P[i] != P[j])) {
      j = next[j];
    }
  }
  
  return next;
}