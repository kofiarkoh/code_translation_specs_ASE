#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct Node{
  int key;
  struct Node *right,*left,*parent;
};

struct Node *root,*NIL;

struct Node* treeMinimum(struct Node *x){
  while(x -> left != NIL) x = x -> left;
  return x;
}

struct Node* find(struct Node *u,int k){
  while(u != NIL && k != u -> key){
    if(k < u -> key) u = u -> left;
    else u = u -> right;
  }
  return u;
}

struct Node* treeSuccessor(struct Node *x){
  struct Node *y;
  if(x -> right != NIL) return treeMinimum(x -> right);
  y = x -> parent;
  while(y != NIL && x == y -> right){
    x = y;
    y = y -> parent;
  }
  return y;
}

void treeDelete(struct Node *z){
  struct Node *y;
  struct Node *x;

  if(z -> left == NIL || z -> right == NIL) y = z;
  else y = treeSuccessor(z);

  if(y -> left != NIL){
    x = y -> left;
  } else{
    x = y -> right;
  }

  if(x != NIL){
    x -> parent = y -> parent;
  }

  if(y -> parent == NIL){
    root = x;
  } else{
    if(y == y -> parent -> left){
      y -> parent -> left = x;
    } else{
      y -> parent -> right = x;
    }
  }

  if(y != z){
    z -> key = y -> key;
  }

  free(y);
}

void insert(int k){
  struct Node *y = NIL;
  struct Node *x = root;
  struct Node *z;

  z = (struct Node*)malloc(sizeof(struct Node));
  z -> key = k;
  z -> left = NIL;
  z -> right = NIL;

  while(x != NIL){
    y = x;
    if(z -> key < x -> key){
      x = x -> left;
    } else{
      x = x -> right;
    }
  }
  z -> parent = y;
  if(y == NIL){
    root= z;
  } else{
    if(z -> key < y -> key){
      y -> left = z;
    } else{
      y -> right = z;
    }
  }
}

void inorder(struct Node *u){
  if(u == NIL) return ;
  inorder(u -> left);
  printf(" %d",u -> key);
  inorder(u -> right);
}

void preorder(struct Node *u){
  if(u == NIL) return ;
  printf(" %d",u -> key);
  preorder(u -> left);
  preorder(u -> right);
}

int main(){
  int n,i,x;
  char str[7];
  struct Node *t;
  scanf("%d",&n);

  for(i = 0;i < n;i++){
    scanf("%s",str);
    if(strcmp(str,"find") == 0){
      scanf("%d",&x);
      t = find(root,x);
      if(t != NIL) printf("yes\n");
      else printf("no\n");
    }
    else if(strcmp(str,"insert") == 0){
      scanf("%d",&x);
      insert(x);
    }
    else if(strcmp(str,"print") == 0){
      inorder(root);
      printf("\n");
      preorder(root);
      printf("\n");
    }
    else if(strcmp(str,"delete") == 0){
      scanf("%d",&x);
      treeDelete(find(root,x));
    }
  }
  return 0;
}