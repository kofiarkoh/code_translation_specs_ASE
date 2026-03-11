#include<stdio.h>
#include<stdlib.h>
#include<string.h>

struct node{
  unsigned int key;
  struct node *nxt;
  struct node *prv;
};

typedef struct node * NodePointer;

NodePointer nu;

NodePointer listSearch(int key) {
  NodePointer c = nu->nxt;
  while( c != nu && c->key != key ) {
    c = c->nxt;
  }
  return c;
}

void mos(){
  nu = malloc(sizeof(struct node));
  nu->nxt = nu;
  nu->prv = nu;
}

void printList(){
  NodePointer c = nu->nxt;
  int m = 1;
  while(1){
    if ( c == nu ) break;
    if ( m == 0)  printf(" ");
    printf("%d", c->key);
    c = c->nxt;
    m = 0;
  }
  printf("\n");
}

void deleteNode(NodePointer t){
  t->prv->nxt = t->nxt;
  t->nxt->prv = t->prv;
}

void delete1(){
  NodePointer t = nu->nxt;
  if ( t == nu ) return;
  deleteNode(t);
}

void delete2(){
  NodePointer t = nu->prv;
  if ( t == nu ) return;
  deleteNode(t);
}

void delete(int key){
  NodePointer t = listSearch(key);
  if ( t == nu ) return;
  deleteNode(t);
}


void insert(int key){
  NodePointer x;
  x = malloc(sizeof(struct node));
  x->key = key;
  x->nxt = nu->nxt;
  nu->nxt->prv = x;
  nu->nxt = x;
  x->prv = nu;
}

int main(){
  int key, n, i;
  int size = 0;
  char A[20];
  int np = 0, nd = 0;
  scanf("%d", &n);
  mos();
  for ( i = 0; i < n; i++ ){
    scanf("%s%d",A, &key);
    if(!( 0 <= key && key <= 2000000000)){
      printf("error");
    }
    if ( A[0] == 'i' ) {
      insert(key);
      np++;
      size++;
    }
    else if ( A[0] == 'd' ) {
      if (strlen(A) > 6){
	if ( A[6] == 'F' ) delete1();
	else if ( A[6] == 'L' ) {
	  delete2();
	}
      }
      else {
	delete(key); nd++; 
      }
      size--;
    }
    if ( size > 1000000 ) {
      printf("error");
    }
  }

  
  if(!( np+nd <= 2000000)){
    printf("error");
  }
  if(!( nd <= 20)){
    printf("error");
  }
  printList();


  return 0;
}

