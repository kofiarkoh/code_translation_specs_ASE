#include<stdio.h>
#define MAX 100005
#define NIL -1
struct Node_struct{int p, l, r;}; /*p: 親(parent), l: 一番左の子(left child), r:右の兄弟(right sibling)*/
typedef struct Node_struct Node;
Node T[MAX];
int n,D[MAX];

void print(int u){
  int i, c;
  printf("node %d: parent = %d, depth = %d, ", u, T[u].p, D[u]);

  if(T[u].p == NIL) printf("root, ");
  else if(T[u].l == NIL) printf("leaf, ");
  else printf("internal node, ");

  printf("[");
  for (i=0, c = T[u].l; c != NIL; i++, c = T[c].r){
    if(i) printf(", ");
    printf("%d", c);
  }
  printf("]\n");

}

void calcDepth(int u, int p){
  D[u] = p;
  if(T[u].r != NIL) calcDepth(T[u].r,p);
  if(T[u].l != NIL) calcDepth(T[u].l,p+1);
}

int main(){
  int i, j, k;
  int id, sib, child;
  int root = NIL;
  scanf("%d", &n);
  
  for(i=0; i < n;i++)T[i].p = T[i].l = T[i].r = NIL;

  for(i=0; i < n;i++){
    scanf("%d %d", &id, &k);
    for(j = 0, sib = NIL; j < k;j++){
      scanf("%d", &child);
      T[child].p = id;
      if(j == 0) T[id].l = child; //idの一番左の子 (the most left child of id)
      else T[sib].r = child; //左の兄弟に繋げる (connect to left sibling)
      sib = child;//次の子供の左の兄弟 (the left sibling of the next child)
    }
  }

  //根を求める (find the root)
  for(i=0; i < n;i++){
    if(T[i].p == -1) root = i;
  }

  //それぞれのノードの深さを求める (find depth of each node)
  calcDepth(root, 0);

//出力 (output)
  for(i=0; i < n;i++){
    print(i);
  }

  return 0;
}

