#include <stdio.h>

const int dx[4] = {-1, 0, 0, 1};
const int dy[4] = {0, -1, 1, 0};

int map[12][12];

void dfs(int y, int x)
{
  int i;
  if (y < 0 || x < 0 || 11 < y || 11 < x){
    return;
  }
  if (map[y][x] == 0){
    return;
  }

  map[y][x] = 0;
  for (i = 0; i < 4; i++){
    dfs(y + dy[i], x + dx[i]);
  }
}

int main()
{
  int i, j;

  while (1){
    for (i = 0; i < 12; i++){
      for (j = 0; j < 12; j++){
        if (scanf("%1d", &map[i][j]) == EOF) {
          return (0);
        }
      }
    }

    int cnt = 0;
    for (i = 0; i < 12; i++){
      for (j = 0; j < 12; j++){
        if (map[i][j] == 0) continue;
        cnt++;
        dfs(i, j);
      }
    }

    printf("%d\n", cnt);
    scanf("%*c");
  }
}