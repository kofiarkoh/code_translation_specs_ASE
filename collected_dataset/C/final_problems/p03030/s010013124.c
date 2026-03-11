#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

typedef struct{
  char CITY[11]; int P; int ANS;
}guide;

int main(void){
  int i, N, j, maxp, a;
  char minc[11];
  guide G[101];
  //読み込み
  scanf("%d", &N);
  for(i=0; i<N; i++){
    scanf("%s %d", G[i].CITY, &G[i].P);
    G[i].ANS = i+1;
  }
  //小さい都市名を見つけて、Pの比較
  for(i=0; i<N-1; i++){
    maxp = G[i].P; strcpy(minc, G[i].CITY); a=i;
    for(j=i+1; j<N; j++){
      if(strcmp(G[j].CITY, minc) < 0){
        maxp = G[j].P;
        strcpy(minc, G[j].CITY);
        a=j;
      }
      else if(strcmp(G[j].CITY, minc) == 0){
        if(G[j].P > maxp){
          maxp = G[j].P;
          a=j;
        }
      }
    }
    G[100] = G[i];
    G[i] = G[a];
    G[a] = G[100];
  }
  for(i=0; i<N; i++) {
    printf("%d\n", G[i].ANS);
  }
  return 0;
}
