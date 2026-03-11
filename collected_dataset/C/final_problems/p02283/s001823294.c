#include <stdio.h>
#include <stdlib.h>
struct node{
  struct node *right;
  struct node *left;
  struct node *parent;
  int key;
};
typedef struct node * Node;
#define NIL NULL
Node root;
Node treeMi(Node a){
  while(a->left!=NIL)
   a=a->left;
  return a;
}
Node treeMa(Node a){
  while(a->right!=NIL){
    a=a->right;
  }
  return a;
}
Node treeS(Node w, int r){
  while(w!=NIL&&r!=w->key){
    if(r<w->key)w=w->left;
    else w=w->right;
  }
  return w;
}
Node treeM(Node a){
  Node b;
  if(a->right!=NIL)
    return treeMi(a->right);
  b=a->parent;
  while(b!=NIL&&a==b->right){
    a=b;
    b=b->parent;
  }
  return b;
}
void treeN(Node c){
  Node a;
  Node b;
  if(c->left==NIL||c->right==NIL)
    b=c;
  else
    b=treeM(c);
  if(b->left!=NIL){
    a=b->left;
  }
  else{
    a=b->right;
  }
  if(a!=NIL){
    a->parent=b->parent;
  }
  if(b->parent==NIL){
    root=a;
  }
  else{
    if(b==b->parent->left){
      b->parent->left=a;
    }
    else{
      b->parent->right=a;
    }
  }
  if(b!=c){
    c->key=b->key;
  }
  free(b);
}
void insert(int k){
  Node a = root;
  Node b;
  Node c;
  c = malloc(sizeof(struct node));
  c->key = k;
  c->left = NIL;
  c->right = NIL;
  while(a!=NIL){
    b=a;
    if(c->key < a->key){
      a=a->left;
    }
    else{
      a=a->right;
    }
  }
  c->parent=b;
  if(b==NIL){
    root=c;
  }
  else{
    if(c->key < b->key){
      b->left=c;
    }
    else{
      b->right=c;
    }
  }
}
void inorder(Node n){
  if(n==NIL)
    return;
  inorder(n->left);
  printf(" %d",n->key);
  inorder(n->right);
}
void outorder(Node n){
  if(n==NIL)
    return;
  printf(" %d",n->key);
  outorder(n->left);
  outorder(n->right);
}
int main(){
  int i,n,a;
  char c[20];
  scanf("%d",&n);

  for(i=0;i<n;i++){
    scanf("%s",c);
    if(c[0]=='f'){
      scanf("%d",&a);
      Node r = treeS(root,a);
      if(r!=NIL)
	printf("yes\n");
      else printf("no\n");
    }
    else if(c[0] ==  'i'){
      scanf("%d",&a);
      insert(a);
    }
    else if(c[0] ==  'p'){
      inorder(root);
      printf("\n");
      outorder(root);
      printf("\n");
    }
    else if(c[0]=='d'){
      scanf("%d",&a);
      treeN(treeS(root,a));
    }
  }
  return 0;
}

