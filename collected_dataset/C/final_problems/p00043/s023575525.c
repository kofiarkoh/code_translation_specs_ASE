#include <stdio.h>

char p[14], result[14], used[14];
char ans_buf[9], top = 0;
int solved;

void push(char c) { ans_buf[top++] = c; }

#define biCh() (result[0] != result[1])
#define triCh(n) (result[n-2] == result[n-1] && result[n-1] == result[n] ? 0 : (result[n-2] == result[n-1] + 1 && result[n-1] == result[n] + 1) ? 0 : 1)

/* int triCh(char n) {
 *   if (result[n-2] == result[n-1] && result[n-1] == result[n]) return 0;
 *   if (result[n-2] == result[n-1] + 1 &&
 *       result[n-1] == result[n] + 1) return 0;
 *   return 1;
 * } */

void calc(char n) {
  if (solved) return;
  if (n == 2 && biCh()) return;
  if (n == 5 && triCh(4)) return;
  if (n == 8 && triCh(7)) return;
  if (n == 11 && triCh(10)) return;
  if (n < 14) {
    int i;
    for (i = 0; i < 14 && !solved; i++) {
      if (used[i]) { continue; }
      used[i] = 1;
      result[n] = p[i];
      calc(n + 1);
      used[i] = 0;
    }
  } else {
    if (triCh(13)) return;
    solved = 1;
    push(p[13]);
  }
}

int dupCh(char n) {
  int i, count;
  for (i = count = 0; i < 13; i++) { if (p[i] == n) { count++; } }
  return !(count < 4);
}
void init() {
  int i;
  for (i = 0; i < 13; i++) { p[i] -= '0'; used[i] = 0; }
  top = p[13] = used[13] = 0;
}

void solve() {
  int i;
  for (i = 1; i <= 9; i++) {
    if (dupCh(i)) { continue; }
    p[13] = i;
    solved = 0;
    calc(0);
  }
}

void printResult() {
  if (!top) {
    puts("0");
  } else {
    int i;
    putchar(ans_buf[0] + '0');
    for (i = 1; i < top; i++) {
      printf(" %c", ans_buf[i] + '0');
    }
    puts("");
  }
}

int main() {
  for (;~scanf("%s\n", p);) {
    init();
    solve();
    printResult();
  }
  return 0;
}