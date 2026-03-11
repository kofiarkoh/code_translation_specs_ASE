#include <stdio.h>
#include <stdlib.h>

#define MAX_HOLE 10000000
// #define MAX_HOLE 1000

typedef struct xy {
  int x, y;
} XY;

static int N = 0;
static XY hole[MAX_HOLE] = {0};

static int memo[MAX_HOLE][MAX_HOLE] = {0};

void memo_clear();
int memo_count();
int countup(int now_a, int now_b);
int d(int i, int j);

int main( void ){
  int i, j, k;
  int a,b;

  scanf("%d %d %d", &N, &a, &b);
  for( i = 0; i < N; i++ ){
    scanf("%d %d", &hole[i].x, &hole[i].y);
  }

  memo_clear();
  countup(a-1, b-1);
  printf( "%d\n", memo_count() );
  // printf( "%d\n", countup(a-1, b-1)+1 );

  return 0;
}

int memo_count(){
  int i,j;
  int count;
  for( i = 0; i < N; i++ ){
    for( j = 0; j < N; j++ ){
      if( memo[i][j] == 1 ){
        count++;
      }
    }
  }
  return count/2;
}

void memo_clear(){
  int i,j;
  for( i = 0; i < N; i++ ){
    for( j = 0; j < N; j++ ){
      memo[i][j] = 0;
    }
  }
}

int d(int i, int j){
  return abs(hole[i].x-hole[j].x)+abs(hole[i].y-hole[j].y);
}

int countup(int now_a, int now_b){
  int i;
  int count = 0;
  // // // // // // // // // printf( "%d %d\n", now_a+1, now_b+1 );
  for( i = 0; i < N; i++ ){
    if( i != now_a && i != now_b ){
      // printf( "d(now) = %d\n", d(now_a, now_b) );
      // printf( "d(next1) = %d\n", d(now_a, i) );
      // printf( "d(next2) = %d\n", d(i, now_b) );
      // printf( "memo1 = %d\n", memo[now_a][i] );
      if( ( d(now_a, now_b) == d(now_a, i) ) &&
          ( memo[now_a][i] == 0 ) ){
        // printf( "a\n" );
        // printf( "yes\n" );
        memo[now_a][i] = 1;
        memo[i][now_a] = 1;
        count = countup(now_a, i) + 1;
      }
      // printf( "memo2 = %d\n", memo[i][now_b] );
      if( ( d(now_a, now_b) == d(i, now_b) ) &&
          ( memo[now_b][i] == 0 ) ){
        // printf( "b\n" );
        // printf( "no\n" );
        memo[i][now_b] = 1;
        memo[now_b][i] = 1;
        count = countup(i, now_b) + 1;
      }
    }
  }
  return count==0 ? 1 : count;
}
