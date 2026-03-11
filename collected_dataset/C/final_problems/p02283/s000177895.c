#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct node *link;
struct node{
  int key;
  link le,ri,pa;
};
link head;

void insert(int a);
void inorder(link h);
void preorder(link h);

int main(){
  int m,k;
  char op[10];
  int i;

  head=NULL;
  scanf("%d",&m);
  for(i=0;i<m;i++){
    scanf("%s",op);
    if(strcmp(op,"insert")==0){
      scanf("%d",&k);
      insert(k);
    }else if(strcmp(op,"print")==0){
      inorder(head);
      printf("\n");
      preorder(head);
      printf("\n");
    }
  }

  return 0;
}

void insert(int a){
  link x,y,z;
  z=malloc(sizeof(struct node));
  z->key=a; z->le=NULL; z->ri=NULL;
  y=NULL;  x=head;
  while(x!=NULL){
    y=x;
    if(z->key<x->key)   x=x->le;
    else                x=x->ri;
  }
  z->pa=y;

  if(y==NULL)             head=z;
  else if(z->key<y->key)  y->le=z;
  else                    y->ri=z;
}

void inorder(link h){
  if(h==NULL) return;
  inorder(h->le);
  printf(" %d",h->key);
  inorder(h->ri);
}

void preorder(link h){
  if(h==NULL) return;
  printf(" %d",h->key);
  preorder(h->le);
  preorder(h->ri);
}
