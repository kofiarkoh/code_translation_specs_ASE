#include <stdio.h>
#include <stdlib.h>

typedef struct vector2 {
  int x;
  int y;
}vector2;

int qcount;
vector2 *queue;
int qfirst = 0, qlast = 0;

void makeQueue(int count) {
  qcount = count;
  qfirst = qlast = count / 2;
  queue = malloc(count * sizeof(vector2));
}

void push_back(int x, int y) {
  vector2 v = { x, y };
  qlast %= qcount;
  queue[qlast++] = v;
}

void push_front(int x, int y) {
  vector2 v = { x, y };
  qfirst = (qfirst + qcount) % qcount;
  queue[--qfirst] = v;
}

vector2 dequeue(void) {
  qfirst %= qcount;
  vector2 v = queue[qfirst++];
  return v;
}

int main(void){
  int h, w;
  scanf("%d%d", &h, &w);
  int ch, cw;
  scanf("%d%d", &ch, &cw);
  ch++, cw++;
  int dh, dw;
  scanf("%d%d", &dh, &dw);
  dh++, dw++;
  makeQueue((h+4) * (w+4));
  char s[h+4][w+4];
  int d[h+4][w+4];
  
  // clear map
  for(int i = 0;i < h + 4;i++) {
    for(int j = 0;j < w + 4;j++) {
      d[i][j] = 0x7FFFFFFF;
      s[i][j] = 0;
    }
  }

  // read map
  for(int i = 2;i < h + 2;i++) {
    scanf("%s", &s[i][2]);
    for(int j = 2;j < w + 2;j++) {
      if(s[i][j] == '.') s[i][j] = 1;
      if(s[i][j] == '#') s[i][j] = 0;
    }
  }

  push_back(cw, ch);
  d[ch][cw] = 0;

  while(qfirst != qlast) {

    vector2 point = dequeue();
    int x = point.x, y = point.y;
    if(x == dw && y == dh) break;
    int data = d[y][x];

    // work
    int dx[] = { -1, 1, 0, 0 };
    int dy[] = { 0, 0, -1, 1 };
    for(int i = 0;i < 4;i++) {
      int sw = x + dx[i], sh = y + dy[i];
      int sd = d[sh][sw];
      // wall
      if(s[sh][sw] == 0) continue;

      if(sd > data) {
        d[sh][sw] = data;
        push_front(sw, sh);
      }
    }

    // warp
    for(int i = -2;i <= 2;i++) {
      for(int j = -2;j <= 2;j++) {
        int sw = x + j, sh = y + i;
        int sd = d[sh][sw];
        // wall
        if(s[sh][sw] == 0) { continue; }

        if(sd > data + 1) {
          d[sh][sw] = data + 1;
          push_back(sw, sh);
        }
      }
    }
    // printf("x: %d y: %d\n", x, y);
    // for(int i = 0;i < h + 4;i++) {
    //   for(int j = 0;j < w + 4;j++) {
    //     printf("%10d ", d[i][j]);
    //   }
    //   puts("");
    // }
  }
  if(d[dh][dw] == 0x7FFFFFFF) {
    puts("-1");
  } else {
    printf("%d\n", d[dh][dw]);
  }
  return 0;
}
