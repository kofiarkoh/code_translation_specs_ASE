#include <stdio.h>
#define limit 200000

int N,K,L;
int douro[limit];
int tetudou[limit];
int drank[limit];
int trank[limit];


void init(int N){
  for(int i = 0;i < N;i++){
    douro[i] = i;
    tetudou[i] = i;
    drank[i] = 0;
    trank[i] = 0;
  }
}

int droot(int x){
  if(x == douro[x]) return x;
  return douro[x] = droot(douro[x]);
}

int dsame(int x,int y){
  if(droot(x) == droot(y))
    return 1;
  return 0;
}

void dunite(int x,int y){
  x = droot(x);
  y = droot(y);
  if(x == y) return;

  if(drank[x] < drank[y]){
    douro[x] = y;
  }else{
    douro[y] = x;
    if(drank[x] == drank[y]) drank[x]++;
  }
}

int troot(int x){
  if(x == tetudou[x]) return x;
  return tetudou[x] = troot(tetudou[x]);
}

int tsame(int x,int y){
  if(troot(x) == troot(y))
    return 1;
  return 0;
}

void tunite(int x,int y){
  x = troot(x);
  y = troot(y);
  if(x == y) return;

  if(trank[x] < trank[y]){
    tetudou[x] = y;
  }else{
    tetudou[y] = x;
    if(trank[x] == trank[y]) trank[x]++;
  }
}

int main(void){
  int p,l,r,s;
  scanf("%d%d%d",&N,&K,&L);

  init(N);
  for(int i = 0;i < K;i++){
    scanf("%d%d",&p,&l);
    p--; l--;
    dunite(p,l);
  }
  for(int i = 0;i < L;i++){
    scanf("%d%d",&r,&s);
    r--; s--;
    tunite(r,s);
  }

  for(int i = 0;i < N;i++){
    int ans = 1;
    for(int j = 0;j < N;j++){
      if(i == j) continue;
      if(tsame(i,j) && dsame(i,j)) ans++;
    }
    printf("%d ",ans);
  }
  printf("\n");

  return 0;
}