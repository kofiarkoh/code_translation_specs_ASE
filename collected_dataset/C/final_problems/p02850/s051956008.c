#include <stdio.h>
#include <stdlib.h>

int N;
int nextNum[114514];
int* nexts[114514];
int* henIndice[114514];

int answer[114514];

void asumikana(int idx, int pidx, int pcolor) {
	int i;
	int nextColor = 1;
	for (i = 0; i < nextNum[idx]; i++) {
		if (nexts[idx][i] == pidx) continue;
        int color;
        if (nextColor == pcolor) nextColor++;
        color = nextColor++;
        answer[henIndice[idx][i]] = color;
        asumikana(nexts[idx][i], idx, color);
	}
}

int main() {
  int i,K;
  if(scanf("%d",&N)!=1)return 1;
  for(i=1;i<N;i++){
    int a,b;
    if(scanf("%d%d",&a,&b)!=2)return 1;

    nexts[a] = realloc(nexts[a], sizeof(int) * (nextNum[a] + 1));
		nexts[a][nextNum[a]] = b;
		henIndice[a] = realloc(henIndice[a], sizeof(int) * (nextNum[a] + 1));
		henIndice[a][nextNum[a]] = i;
		nextNum[a]++;

    nexts[b] = realloc(nexts[b],sizeof(int)*(nextNum[b]+1));
    nexts[b][nextNum[b]]=a;
    henIndice[b]=realloc(henIndice[b],sizeof(int)*(nextNum[b]+1));
    henIndice[b][nextNum[b]]=i;
    nextNum[b]++;
  }
  K=0;
  for(i=1;i<=N;i++){
    if(nextNum[i]>K)K=nextNum[i];
  }
  asumikana(1,0,0);

  printf("%d\n",K);
  for(i=1;i<N;i++){
    printf("%d\n",answer[i] );
  }

  for(i=1;i<=N;i++){
    free(nexts[i]);
    free(henIndice[i]);
  }
  return 0;
}
