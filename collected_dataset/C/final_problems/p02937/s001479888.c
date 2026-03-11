#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define N 100000
int A[26][N]={0}, pos[26]={0};

int  main(){
  int l, i, tmp, ex, cur, j;
  long long int loop=0;
  char s[N], t[N];
  scanf("%s", s);
  scanf("%s", t);
  l=strlen(s);
  for (i=0; i<l; i++){
    tmp=s[i]-'a';
    A[tmp][pos[tmp]]=i; pos[tmp]++;
  }

/*
  for (i=0; i<26; i++){
    printf("%d ", pos[i]);
  }
*/

  cur=-1;
  for (i=0; i<strlen(t); i++){
//    printf("%lld %d\n", loop, cur);
    tmp=t[i]-'a';
//    printf("%c\n", tmp+'a');
    ex=0;
    if (pos[tmp]==0) {
      printf("-1\n"); return 0;
    }
    for (j=0; j<pos[tmp]; j++){
      if (A[tmp][j]>cur) {
        ex=1; cur=A[tmp][j]; break;
      }
    }
    if (ex) continue;
    cur=A[tmp][0]; loop++;
  }
  printf("%lld\n", loop*strlen(s)+cur+1);
  return 0;
}
