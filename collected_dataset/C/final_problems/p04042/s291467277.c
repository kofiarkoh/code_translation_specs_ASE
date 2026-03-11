#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define NMAX 2000
#define KMAX 10000

int n;
int k;
char s[NMAX][KMAX+3];  // \r\n\0
char* part[KMAX+1];

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
  }
}

void buildPart() {
  int i, j;
  static char tmp[KMAX+1];
  for (j = 0; j < k; j++) { part[j] = 0; }
  part[k] = (char*)malloc(1);
  part[k][0] = '\0';
  for (i = n-1; i >= 0; i--) {
    int len = strlen(s[i]);
    for (j = 0; j < k; j++) {
      if ((j + len <= k) && (part[j + len] != 0)) {
	strcpy(tmp, s[i]);
	strcat(tmp, part[j + len]);
	if (part[j] == 0 || strcmp(tmp, part[j]) < 0) {
	  if (part[j] == 0) {
	    part[j] = (char*)malloc(k-j+1);
	  }
	  strcpy(part[j], tmp);
	}
      }
    }
  }
}

int main() {
  readQ();
  int i;
  buildPart();
  printf("%s\n", part[0]);
  return 0;
}

