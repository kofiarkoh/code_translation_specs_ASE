#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define C 1000000007
#define NMAX 2000
#define KMAX 10000


int n;
int k;
char s[NMAX][KMAX+3];  // \r\n\0
int len[NMAX];
int o[NMAX];
char map[NMAX+1][KMAX+1];

void readQ() {
  static char buf[80];

  if (!fgets(buf, 80, stdin)) { perror("internal error #1"); }
  sscanf(buf, "%d %d", &n, &k);

  int i;
  for (i = 0; i < n; i++) {
    if (!fgets(s[i], KMAX+3, stdin)) { perror("internal error #2"); }
    int m = strlen(s[i]);
    if (s[i][m-1] == '\n') { s[i][m-1] = '\0'; }
    if (s[i][m-2] == '\r') { s[i][m-2] = '\0'; }
    len[i] = strlen(s[i]);
  }
}


int compare(const void *p, const void *q) {
  return strcmp(s[*(const int*)p], s[*(const int*)q]);
}

void buildMap() {
  int i, j;

  map[n][0] = 1;
  for (j = 1; j < k; j++) { map[n][j] = 0; }
  for (i = n - 1; i >= 0; i--) {
    for (j = 0; j <= k; j++) {
      if (map[i+1][j]) {
	map[i][j] = 1;
	if (j + len[i] <= k) { map[i][j + len[i]] = 1; }
      }
    }
  }
}

int next_idx(int lower_bound, int length, int i) {
  for ( ; i < n; i++) {
    int p = o[i];
    if (p < lower_bound) { continue; }
    int newlen = length - len[p];
    if (!map[p+1][newlen]) { continue; }
    return i;
  }
  return -1;
}

char* mkstr(int start, int length) {
  char* buf = (char*)malloc(length+1);
  char* buf2 = (char*)malloc(length+1);

  int first = 1;
  int i = -1;
  while (1) {
    i = next_idx(start, length, i + 1);
    if (i < 0) { break; }
    int p = o[i];
    if (!first && strcmp(s[p], buf) >= 0) { break; }

    strcpy(buf2, s[p]);
    int newlen = length - len[p];
    if (newlen > 0) {
      char* z = mkstr(o[i] + 1, newlen);
      strcat(buf2, z);
      free(z);
    }
    if (first || strcmp(buf2, buf) < 0) {
      strcpy(buf, buf2);
      first = 0;
    }
  }
  if (first) { perror("internal error #3"); }
  return buf;
}

int main() {
  readQ();
  int i;
  for (i = 0; i < n; i++) { o[i] = i; }
  qsort(o, n, sizeof(int), compare);
  buildMap();
  char* a = mkstr(0, k);
  printf("%s\n", a);
  free(a);
  return 0;
}
