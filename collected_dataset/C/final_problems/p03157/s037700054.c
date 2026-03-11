#include <stdio.h>

int main(void) {

  int h,w;
  scanf("%d %d", &h, &w);
  char s[h][w+1];
  for (int i = 0; i < h; i++) {
    scanf("%s", s[i]);
  }
  int visited[h*w],queue[h*w];
  for (int i = 0; i < h*w; i++) {
    visited[i] = 0;
    queue[i] = -1;
  }
  visited[0] = 1;
  queue[0] = 0;
  int head = 0, tail = 0, start = 0, end = 0, cur, flag;
  int ans = 0;
  int black = 0, white = 0;
  if (s[0][0] == '#') {
    black++;
  } else {
    white++;
  }
  while (1) {
    flag = 0;
    for (int i = start; i <= end; i++) {
      cur = queue[i];
      if (cur%w != w-1 && visited[cur+1] == 0 && s[cur/w][cur%w] != s[cur/w][cur%w+1]) {
        if (s[cur/w][cur%w+1] == '#') {
          black++;
        } else {
          white++;
        }
        visited[cur+1] = 1;
        tail++;
        queue[tail] = cur+1;
        flag = 1;
      }
      if (cur/w != h-1 && visited[cur+w] == 0 && s[cur/w][cur%w] != s[cur/w+1][cur%w]) {
        if (s[cur/w+1][cur%w] == '#') {
          black++;
        } else {
          white++;
        }
        visited[cur+w] = 1;
        tail++;
        queue[tail] = cur+w;
        flag = 1;
      }
    }
    if (flag == 0) {
      ans += black*white;
      if (tail == h*w-1) {
        break;
      } else {
        black = 0;
        white = 0;
        for (int i = head+1; i < h*w; i++) {
          if (visited[i] == 0) {
            visited[i] = 1;
            head = i;
            break;
          }
        }
        tail++;
        queue[tail] = head;
        if (s[head/w][head%w] == '#') {
          black++;
        } else {
          white++;
        }
        start = tail;
        end = tail;
      }
    } else {
      start = end+1;
      end = tail;
    }
  }
  printf("%d\n", ans);

  return 0;
}