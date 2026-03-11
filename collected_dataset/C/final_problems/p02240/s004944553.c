#include<stdio.h>
#include<stdlib.h>
typedef struct list{
  int k, *v, c;
}T_list;
int adj(int, int);
int n, m;
//int **mtr;
T_list *lst;
int *d;
int main(){
  scanf("%d %d", &n, &m);
  //mtr = (int **)malloc(sizeof(int *) * n);
  d = (int *)malloc(sizeof(int) * n);
  lst = (T_list *)malloc(sizeof(T_list) * n);
  int i, j;
  for(i = n - 1; i >= 0; i--){
    //d[i] = 0;
    lst[i].k = 0;
    //mtr[i] = (int *)malloc(sizeof(int) * n);
    //for(j = n - 1; j >= 0; j--)
      //mtr[i][j] = 0;
  }
  int **st;
  st = (int **)malloc(sizeof(int *) * m);
  for(i = m - 1; i >= 0; i--){
    st[i] = (int *)malloc(sizeof(int) * 2);
    scanf("%d %d", &st[i][0], &st[i][1]);
    lst[st[i][0]].k++;
    lst[st[i][1]].k++;
    //mtr[s][t] = mtr[t][s] = 1;
  }
  for(i = n - 1; i >= 0; i--){
    //printf(" %d\n", lst[i].k);
    if(lst[i].k){
      lst[i].v = (int *)malloc(sizeof(int) * lst[i].k);
    }
    //printf("%d", i);
    lst[i].c = 0;
    //printf("%d.", lst[i].c);
  }
  for(i = m - 1; i >= 0; i--){
    //printf(" %d", st[i][0]);
    //if(i < 5)
    lst[st[i][0]].v[lst[st[i][0]].c++] = st[i][1];
    lst[st[i][1]].v[lst[st[i][1]].c++] = st[i][0];
    free(st[i]);
  }
  free(st);
  scanf("%d", &m);
  int s, t;
  for(i = m - 1; i >= 0; i--){
    scanf("%d %d", &s, &t);
    for(j = n - 1; j >= 0; j--)
      d[j] = 0;
    if(adj(s, t))
      printf("yes\n");
    else
      printf("no\n");
  }
  //for(i = n - 1; i >= 0; i--)
    //free(mtr[i]);
  free(lst), free(d);
  return 0;
}

int adj(int s, int t){
  d[s] = 1;
  //printf("%d %d %d\n", v + 1, grp[v].d, lst[v].k);
  int i;
  for(i = lst[s].k - 1; i >= 0; i--){
    if(lst[s].v[i] == t)
      return 1;
    if(!d[lst[s].v[i]]){
      //printf(" %d\n", lst[v].v[i] - 1);
      if(adj(lst[s].v[i], t))
	return 1;
    }
  }

/*  d[s] = 1;
  if(mtr[s][t])
    return 1;
  int i;
  for(i = n - 1; i >= 0; i--)
    if(mtr[s][i] & !d[i])
      if(adj(i, t))
	return 1;*/
  return 0;
}