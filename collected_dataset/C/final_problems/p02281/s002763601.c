#include<stdio.h>
#define MAX 10000
#define NIL -1

struct Node{
  int p, l, r;
};

struct Node TREE[MAX];
int n;

void preParse(int u){
  if ( u == NIL ) return;
  printf(" %d", u);
  preParse(TREE[u].l);
  preParse(TREE[u].r);
}
void inParse(int u){
  if ( u == NIL ) return;
  inParse(TREE[u].l);
  printf(" %d", u);
  inParse(TREE[u].r);
}
void postParse(int u){
  if ( u == NIL ) return;
  postParse(TREE[u].l);
  postParse(TREE[u].r);
  printf(" %d", u);
}



int main(){
  int i, v, l, r, root;
  scanf("%d", &n);
  for ( i = 0; i < n; i++ ) {
    TREE[i].p = NIL;
  }

  for ( i = 0; i < n; i++ ){
    scanf("%d %d %d", &v, &l, &r);
    TREE[v].l = l;
    TREE[v].r = r;
    if ( l != NIL ) TREE[l].p = v;
    if ( r != NIL ) TREE[r].p = v;
  }
  for ( i = 0; i < n; i++ ) if ( TREE[i].p == NIL ) root = i;
  printf("Preorder\n");
  preParse(root);
  printf("\n");
  printf("Inorder\n");
  inParse(root);
  printf("\n");
  printf("Postorder\n");
  postParse(root);
  printf("\n");


  return 0;
}
