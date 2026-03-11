#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define L 20
#define N (1<<L)

char cands[N][L+1];
int cands_ok[N];
int nok;

int main(int argc, char *argv[])
{
  char buf[L+2];
  int i, j, k;
  for(;;){
    int l, n;
    fgets(buf, L+2, stdin);
    l = strlen(buf) - 1;
    buf[l] = 0;  // to remove the newline
    n = 1 << l;
    if(buf[0]=='#') break;
    nok = 0;
    for(i = 0; i < n; i++) {
      char exec[L+1];
      char cand[L+1];
      for(j = 0; j < l; j++) {
        cand[j] = buf[j] + (((1<<(l-1-j)) & i) ? 1 : 0);
        exec[j] = cand[j];
      }
      cand[l] = 0;
      exec[l] = 0;
      //printf("cand = %s\n", cand);
      for(k = 0; k < 25; k++) {
        char c1 = 'a' + k;
        char c2 = c1 + 1;
        for(j = 0; j < l; j++) {
          if(exec[j] == c2) {
            exec[j] = c1;
            break;
          }
        }
      }
      //printf("exec = %s\n", exec);
      if(strcmp(exec, buf) == 0) {
        cands_ok[nok] = i;
        nok++;
        char *candsi = cands[i];
        for(j = 0; j <= l; j++) {
          candsi[j] = cand[j];
        }
      }
    }
    printf("%d\n", nok);
    if(nok < 10) {
      for(i = 0; i < nok; i++) {
        printf("%s\n", cands[cands_ok[i]]);
      }
    } else {
      for(i = 0; i < 5; i++) {
        printf("%s\n", cands[cands_ok[i]]);
      }
      for(i = nok - 5; i < nok; i++) {
        printf("%s\n", cands[cands_ok[i]]);
      }
    }
  }
  return 0;
}