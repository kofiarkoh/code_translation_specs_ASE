#include <stdio.h>

int main(void) {

  int d;
  scanf("%d", &d);
  long c[26];
  for (int i = 0; i < 26; i++) {
    scanf("%ld", &c[i]);
  }
  long s[d][26];
  for (int i = 0; i < d; i++) {
    for (int j = 0; j < 26; j++) {
      scanf("%ld", &s[i][j]);
    }
  }
  int t[d];
  for (int i = 0; i < d; i++) {
    scanf("%d", &t[i]);
  }
  int last[26];
  for (int i = 0; i < 26; i++) {
    last[i] = -1;
  }
  long score = 0;
  for (int i = 0; i < d; i++) {
    score += s[i][t[i]-1];
    last[t[i]-1] = i;
    for (int j = 0; j < 26; j++) {
      if (j == t[i]-1) {
        continue;
      }
      score -= c[j]*(i-last[j]);
    }
  }
  int m,q_d,q_q;
  scanf("%d", &m);
  for (int que = 0; que < m; que++) {
    scanf("%d %d", &q_d, &q_q);
    t[q_d-1] = q_q;
    for (int i = 0; i < 26; i++) {
      last[i] = -1;
    }
    score = 0;
    for (int i = 0; i < d; i++) {
      score += s[i][t[i]-1];
      last[t[i]-1] = i;
      for (int j = 0; j < 26; j++) {
        if (j == t[i]-1) {
          continue;
        }
        score -= c[j]*(i-last[j]);
      }
    }
    printf("%ld\n", score);
  }

  return 0;
}