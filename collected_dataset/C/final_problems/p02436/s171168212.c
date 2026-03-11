#include <stdio.h>
#include <stdlib.h>

int main()
{
  int i,  n, q, nq, t, x;
  fscanf(stdin, "%d %d", &n, &nq);
  int **queue = (int**)malloc(sizeof(int*)*n);
  int *head = (int*)malloc(sizeof(int)*n);
  int *tail = (int*)malloc(sizeof(int)*n);
  for (i=0; i<n; i++) {
    queue[i] = (int*)malloc(sizeof(int)*nq);
    head[i]=0;
    tail[i]=0;
  }
  
  for (i=0; i<nq; i++) {
    fscanf(stdin, "%d %d", &q, &t);
    switch (q) {
    case 0:
      fscanf(stdin, "%d", &x);
      queue[t][(tail[t]++ % nq)] = x;
      break;
    case 1:
      if (head[t]!=tail[t])
	printf("%d\n", queue[t][head[t] % nq]);
      break;
    case 2:
      //      if (head[t]%nq!=tail[t]%nq)
      if (head[t] != tail[t])
	head[t]++;
    }
  }
  return 0;
}

