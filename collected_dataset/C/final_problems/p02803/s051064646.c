#include <stdio.h>

int H, W;
int sx, sy;
char map[100][100]; // Y X                                                                                                      
int dist[100][100]; // Y X                                                                                                      

void clearDist() { for (int i = 0; i < H; i++) for (int j = 0; j < W; j++) dist[i][j] = -1; }

int list_top; // いま埋めようとしている                                                                                         
struct { int x, y, count; } todo[10240]; // 埋めるリスト                                                                        

static int set(int x, int y, int count) {
  if (x < 0 || y < 0 || x >= W || y >= H) return 0;
  if (map[y][x] != '.') return 0;
  if (dist[y][x] >= 0) return 0;
  todo[list_top].x = x; todo[list_top].y = y; dist[y][x] = count;
  todo[list_top].count = count+1;
  list_top++;
  return 1;
}

void provision(int x, int y) {
  int i = 0, count;
  list_top = 1;
  todo[0].x = x; todo[0].y = y; todo[0].count = 1;
  dist[y][x] = 0;
  for (i = 0; i < list_top; i++) {
    x = todo[i].x; y = todo[i].y; count = todo[i].count;
    set(x-1,y,count);
    set(x+1,y,count);
    set(x,y-1,count);
    set(x,y+1,count);
  }
}
int findMax() {
  int max = 0;
  for (int i = 0; i < H; i++) for (int j = 0; j < W; j++) {
      if (max < dist[i][j]) { sx = j; sy = i; max = dist[i][j]; }
    }
  return max;
}
void findStartPos() {
  for (int i = 0; i < H; i++) for (int j = 0; j < W; j++) {
      if (map[i][j] == '.') { sx = j; sy = i; return; }
    }
}
int main() {
  int i, max, max2;
  scanf("%d %d", &H, &W);
  for (i = 0; i < H; i++) {
    scanf("%s", &(map[i][0]));
  }
  findStartPos(); max = 0;
  do {
    max2 = max; clearDist(); provision(sx, sy); max = findMax();
  } while (max2 != max);
  max = findMax();
  printf("%d\n", max);
}
