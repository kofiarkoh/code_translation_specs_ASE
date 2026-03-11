#include<stdio.h>
#include<stdlib.h>

int Puzzle[4][4];
int tmp[4][4];
int dx[4] = {0, 1, 0, -1};
int dy[4] = {1, 0, -1, 0};

void Empty(int *, int *);
void swap(int *, int *);
int Cal();
int DFS(int, int, int, int, int);

int main(){
  int i, j, l;
  int x, y, a;

  for(i = 0 ; i < 4 ; i++){
    for(j = 0 ; j < 4 ; j++){
      scanf("%d",&Puzzle[i][j]);
    }
  }

  Empty(&x,&y);

  for(l = 0 ; l < 100 ; l++){
    for(i = 0 ; i < 4 ; i++){
      for(j = 0 ; j < 4 ; j++){
	tmp[i][j] = Puzzle[i][j];
      }
    }

    a = DFS(x, y, -1, 0, l);
    if(a != -1){
      printf("%d\n",a);
      break;
    }

    for(i = 0 ; i < 4 ; i++){
      for(j = 0 ; j < 4 ; j++){
	Puzzle[i][j] = tmp[i][j];
      }
    }
    
    a = -1;
    if(l == 99) printf("%d\n",a);
  }

  return 0;
}

void Empty(int *x, int *y){
  int i, j;

  for(i = 0 ; i < 4 ; i++){
    for(j = 0 ; j < 4 ; j++){
      if(!Puzzle[i][j]){
	*x = j;
	*y = i;
	return;
      }
    }
  }
}

void swap(int *a, int *b){
  int tmp;
  
  tmp = *a;
  *a = *b;
  *b = tmp;
}


int Cal(){
  int i, a;
  int sum = 0;

  for(i = 0 ; i < 16 ; i++){
    a = Puzzle[i/4][i%4];
    if(a){
      a--;
      sum += abs(i/4 - a/4)  + abs(i%4 - a%4);
    }
  }

  return sum;
}

int DFS(int x, int y, int p, int dep, int limit){
  int i, h, nx, ny, res;

  h = Cal();
  
  if(h == 0) return dep;
  if(dep + h > limit) return -1;

  for(i = 0 ; i < 4 ; i++){
    nx = x + dx[i];
    ny = y + dy[i];

    if(~p && i == (p+2)%4) continue;
    if(!(0 <= nx && nx < 4 && 0 <= ny && ny < 4)) continue;

    swap(&Puzzle[y][x], &Puzzle[ny][nx]);
    res = DFS(nx, ny, i, dep+1, limit);
    if(~res) return res;

    swap(&Puzzle[y][x], &Puzzle[ny][nx]);
  }

  return -1;
}

