#include <stdio.h>
#include <stdlib.h>

#define NIL NULL

struct node {
  struct node *right;
  struct node *left;
  struct node *parent;
  int key;
};
typedef struct node *Node;

Node root;

Node treeMinimum(Node);
Node treeMaximum(Node);
Node treeSearch(Node, int);
Node treeSuccessor(Node);
void treeDelete(Node);
void insert(int);
void inorder(Node);
void preorder(Node);


int main() {
  int n, i, j, x, count=0, *state;
  char com[20];
  Node t;
  
  scanf( "%d" , &n );
  state = malloc(n * sizeof(int));

  for ( i=0; i<n; i++ ) {
    scanf( "%s" , com );
    
    if ( com[0] == 'f' ) {
      scanf( "%d" , &x );
      t = treeSearch(root, x);
      if ( t != NIL ) state[count] = 1;
      else state[count] = 0;
      count++;
    }
    
    else if ( com[0] == 'i' ) {
      scanf( "%d" , &x );
      insert(x);
    }
    
    else if ( com[0] == 'p' ) {
      for ( j=0; j<count; j++ ) {
	if ( state[j] == 1 ) printf( "yes\n" );
	else printf( "no\n" );
      }
      count = 0;
      inorder(root);
      printf( "\n" );
      preorder(root);
      printf( "\n" );
    }
    
    else if ( com[0] == 'd' ) {
      scanf( "%d" , &x );
      treeDelete(treeSearch(root, x));
    }
  }
  for ( j=0; j<count; j++ ) {
    if ( state[j] == 1 ) printf( "yes\n" );
    else printf( "no\n" );
  }
  
  return 0;
}


Node treeMinimum(Node x){
  while ( x->left != NIL ) x = x->left;
  return x;
}

Node treeMaximum(Node x){
  while ( x->right != NIL ) x = x->right;
  return x;
}

Node treeSearch(Node u, int k){
  if ( (u == NIL) || (k == u->key) ) return u;
  if ( k < u->key ) return treeSearch(u->left, k);
  else return treeSearch(u->right, k);
}

Node treeSuccessor(Node x){
  Node y;
  
  if ( x->right != NIL ) return treeMinimum(x->right); // case 1
  y = x->parent;
  while ( (y != NIL) && (x == y->right) ) {
    x = y;
    y = y->parent;
  }
  return y; // case 2
}

void treeDelete(Node z){
  Node y; // node to be deleted
  Node x; // child of y

  if ( (z->left == NIL) || (z->right == NIL) ) y = z;
  else y = treeSuccessor(z);

  if ( y->left != NIL ) x = y->left;
  else x = y->right;

  if ( x != NIL ) x->parent = y->parent;
  
  if ( y->parent == NIL ) root = x;
  else if ( y == y->parent->left ) y->parent->left = x;
  else y->parent->right = x;
  
  if ( y != z ) z->key = y->key;
}

void insert(int k){
  Node y = NIL;
  Node x = root;
  Node z;

  z = malloc(sizeof(struct node));
  z->key = k;
  z->left = NIL;
  z->right = NIL;

  while ( x != NIL ) {
    y = x;
    if ( z->key < x->key ) x = x->left;
    else x = x->right;
    z->parent = y;
  }
  if ( y == NIL ) root = z; // Tree is empty
  else if ( z->key < y->key ) y->left = z;
  else y->right = z;
}

void inorder(Node u){
  if ( u != NIL ) {
    inorder(u->left);
    printf( " %d" , u->key );
    inorder(u->right);
  }
}

void preorder(Node u){
  if ( u != NIL ) {
    printf( " %d" , u->key );
    preorder(u->left);
    preorder(u->right);
  }
}

