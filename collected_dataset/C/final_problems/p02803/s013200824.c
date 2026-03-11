#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <limits.h>
#include <math.h>
int acs(const void *a, const void *b){return *(int*)a - *(int*)b;} /* 1,2,3,4.. */
int des(const void *a, const void *b){return *(int*)b - *(int*)a;} /* 8,7,6,5.. */
int cmp_char(const void *a, const void *b){return *(char*)a - *(char*)b;} /* a,b,c,d.. */
int cmp_str(const void *a, const void *b){return strcmp(*(const char **)a, *(const char **)b);} /* aaa,aab.. */
#define min(a,b) (a < b ? a: b)
#define max(a,b) (a > b ? a: b)
#define rep(i, l, r) for (int i = l; i < r; i++)
#define MAX 100001
#define INF 1000000007

typedef long long int lli;

typedef struct
{
  int front;
  int rear;
  int data[MAX][3];
} queue;

queue que;

void initQueue(queue *p)
{
  p->front = p->rear = 0;
}

bool emptyQueue(queue *p)
{
  return p->front == p->rear;
}

bool fullQueue(queue *p)
{
  return (p->front - 1 == p->rear) || (p->front == 0 && p->rear == MAX - 1);
}

void queuePop(queue *p)
{
  if (emptyQueue(p))
  {
    printf("UnderFlow\n");
    exit(1);
  }
  else
  {
    p->front++;
    if (p->front == MAX)
      p->front = 0;
  }
}

void queuePush(queue *p, int x, int y, int dist)
{
  if (fullQueue(p))
  {
    printf("OverFlow\n");
    exit(1);
  }
  else
  {
    p->data[p->rear][0] = x;
    p->data[p->rear][1] = y;
    p->data[p->rear][2] = dist;
    p->rear++;
    if (p->rear == MAX)
      p->rear = 0;
  }
}


int main(void) {
  char s[22][22];
  int h, w;
  scanf("%d %d", &h, &w);
  rep(hi, 0, h) {
    scanf("%s", s[hi]);
  }

  int ans = 0;
  rep(hi, 0, h) {
    rep(wi, 0, w) {
      if ( s[hi][wi] == '#' ) continue;
      int visited[21][21] = {0};
      initQueue(&que);
      queuePush(&que, hi, wi, 0);
      while ( !emptyQueue(&que) ) {
        int x    = que.data[que.front][0];
        int y    = que.data[que.front][1];
        int dist = que.data[que.front][2];
        queuePop(&que);
        visited[x][y] = 1;
        ans = max(ans, dist);
        int mx[4] = { 0, 1, 0, -1 };
        int my[4] = { 1, 0, -1, 0 };
        rep(mi, 0, 4) {
          int nextX = x + mx[mi];
          int nextY = y + my[mi];
          if ( nextX < 0 || h <= nextX ) continue;
          if ( nextY < 0 || w <= nextY ) continue;
          if ( visited[nextX][nextY] )   continue;
          if ( s[nextX][nextY] == '#' )  continue;
        
          queuePush(&que, nextX, nextY, dist + 1);
        }
      }
    }
  }

  printf("%d\n", ans);
  return 0;
}
