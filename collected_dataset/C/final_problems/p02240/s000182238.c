#include <stdio.h>
#include <stdlib.h>

void search(int **a, int n, char **group, int g_num, int u, int *affiliation) {
  if(group[g_num][u] != 0) return;
  group[g_num][u] = 1;
  affiliation[u] = g_num;
  int i;
  for (i = 0; i < n; i++) {
    if(a[u][i] == 1) {
      search(a,n,group,g_num,i,affiliation);
    }
  }
}

int main()
{
  int i,j,n,m,**a,u,v;
  scanf("%d %d",&n,&m);
  a = (int**)malloc(sizeof(int*) * n);
  for (i = 0; i < n; i++) {
    a[i] = (int*)malloc(sizeof(int) * n);
    for(j = 0; j < n; j++) a[i][j] = 0;
  }
  for (i = 0; i < m; i++) {
    scanf("%d",&u);
    scanf("%d",&v);
    a[u][v] = 1;
    a[v][u] = 1;
 
  }

  char **group;
  group = (char**)malloc(sizeof(char*) * n);
  for (i = 0; i < n; i++) {
    group[i] = (char*)malloc(sizeof(char) * n);
    for(j = 0; j < n; j++) group[i][j] = 0;
  }
  int *affiliation;
  affiliation = (int*)malloc(sizeof(int) * n);
  for (i = 0; i < n; i++) {
    affiliation[i] = -1;
  }

  j = 0;
  for (i = 0; i < n; i++) {
    if(affiliation[i] == -1) {
      search(a,n,group,j,i,affiliation);
      j++;
    }
  }

  int q,s,t;
  scanf("%d",&q);
  for (i = 0; i < q; i++) {
    scanf("%d %d",&s,&t);
    if(group[affiliation[s]][t] == 1) printf("yes\n");
    else printf("no\n");

  }



  for (i = 0; i < n; i++) {
    free(a[i]);
    free(group[i]);
  }
  free(a);
  free(group);
  free(affiliation);
  return 0;
}