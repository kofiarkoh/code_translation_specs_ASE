#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct node {
  struct node *parent;
  int value;
} Node;

int compar(const void *a, const void *b)
{
  return *(int*)b - *(int*)a;
}

int main()
{
  int i, j, slen, ans;
  char s[100003];
  int cc[26] = {0};
  fgets(s, 100002, stdin);
  slen = strlen(s);
  if (s[slen-1]=='\n')  s[--slen]='\0';
  for (i=0; i<slen; i++) cc[s[i]-'a']++;
  qsort(cc, 26, sizeof(int), compar);
  for (j=0; cc[j]!=0; j++);
  if (j==1) {
    printf("%d\n", slen);
    return 0;
  }

  Node *nn = (Node*)malloc(sizeof(Node)*j*2);
  int n0, m0, m1;
  n0 = 0;
  m0 = m1 = j;

  for (i=0; i<j; i++) {
    nn[i].value = cc[j-i-1];
    nn[i].parent = NULL;
  }
  Node *x, *y, *z;
  do {
    if (n0<j && (nn[m0].value==0 || nn[n0].value<nn[m0].value)) {
      x = &nn[n0++];
    } else {
      x = &nn[m0++];
    }
    if (n0<j && (nn[m0].value==0 || nn[n0].value<nn[m0].value)) {
      y = &nn[n0++];
    } else {
      y = &nn[m0++];
    }
    if (y->value==0) break;
    z = &nn[m1++];
    x->parent=z;
    y->parent=z;
    z->value=x->value+y->value;
  } while (j>=n0);

  ans=0;
  for (i=0; i<j; i++) {
    for (z = &nn[i]; z->parent; z=z->parent)
      ans += cc[j-i-1];
  }
  printf("%d\n", ans);
  return 0;
}

