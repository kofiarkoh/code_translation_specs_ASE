#include <stdio.h>

#define N 8
#define NOT_FREE 1
#define FREE 0

void putQueen(int);
void PrintBoard();
int num;
int BLOCK[N][N];
int col[N];
int row[N];
int dpos[15];
int dneg[15];


int main(){
  int i,j,u,k;
  scanf("%d",&num);
  for(i=0;i<num;i++){
    scanf("%d%d",&u,&k);
    BLOCK[u][k]=1;
  }

  putQueen(0);

  return 0;
}

void putQueen(int i){
int j;
  if(i==N){
    PrintBoard();
    return;
  }

  for(j=0;j<N;j++){
    if(col[j]==NOT_FREE || dpos[i+j] == NOT_FREE || dneg[i-j+N-1] == NOT_FREE){
      continue;
    }
      //put queen at (i,j)
      row[i] = j;
      col[j] = dpos[i+j] = dneg[i-j+N-1] = NOT_FREE;
    putQueen(i+1);
    col[j] = dpos[i+j] = dneg[i-j+N-1] = FREE;
  }
}

void PrintBoard(){
  int i,j;
  for(i=0;i<N;i++){
    for(j=0;j<N;j++){
      if(BLOCK[i][j] == 1 && row[i] != j){
        return;
      }
    }
  }
  for(i=0;i<N;i++){
    for(j=0;j<N;j++){
      if(row[i]==j){
        printf("Q");
      }
      else printf(".");
    }
      printf("\n");
  }
}