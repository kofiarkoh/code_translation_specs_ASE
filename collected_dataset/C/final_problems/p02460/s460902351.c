#include <stdio.h>
#include <string.h>
#include <stdlib.h>

const unsigned int B = 9973;

typedef struct node {
  struct node *next;
  char key[21]; //  char *key;
  int value;
} Node;

unsigned int compute_hash(char *a)
{
  unsigned int hash = 0;
  int len = strlen(a);
  for (int i=0; i<len; i++) hash = hash * B + a[i];
  return hash;
}

int main(){
  int n, i, x, q, hash;
  char buf[256];
  char key[21];
  char *p1, *p2;
  fgets(buf, 256, stdin);
  n = atoi(buf);
  Node **a = (Node**)malloc(sizeof(Node*)*n/2);
  for (i = 0; i < n/2; i++)  a[i] = NULL;
  for (i = 0; i < n; i++) {
    fgets(buf, 256, stdin);
    q = atoi(buf);
    p1 = index(buf, ' ') + 1;
    if (q==0) { // insert
      p2 = index(p1, ' ');
      strncpy(key, p1, p2-p1);
      key[p2-p1] = '\0';
      hash = compute_hash(key)%(n/2);
      x = atoi(p2);
      if (a[hash]==NULL) {
	a[hash] = (Node*)malloc(sizeof(Node));
	a[hash]->next = NULL;
	//	a[hash]->key = (char*)malloc(strlen(buf)+1);
	strcpy(a[hash]->key, key);
	a[hash]->value = x;
	continue;
      }
      
      Node *ptr = a[hash];
      Node *prev = ptr;
      while (ptr) {
	if (strcmp(ptr->key, key)==0) {
	  ptr->value = x;
	  break;
	}
	prev = ptr;
	ptr = ptr->next;
      }
      if (ptr==NULL) {
	ptr = (Node*)malloc(sizeof(Node));
	ptr->next = NULL;
	//	ptr->key = (char*)malloc(strlen(buf)+1);
	strcpy(ptr->key, key);
	ptr->value = x;
	prev->next = ptr;
      }
      
    } else if (q==1) { // find
      p2 = index(p1, '\n');
      strncpy(key, p1, p2-p1+1);
      key[p2-p1] = '\0';
      hash = compute_hash(key)%(n/2);
      Node *ptr = a[hash];
      while (ptr) {
	if (strcmp(ptr->key,key)==0) {
	  int b = ptr->value;
	  int j;
	  for (j=1; j*10<=b; j*=10);
	  for (; j>0; j/=10) {
	    putc_unlocked('0'+b/j, stdout);
	    b %= j;
	  }
	  putc_unlocked('\n', stdout);
	  break;
	}
	ptr = ptr->next;
      }
      if (ptr==NULL) {
	putc_unlocked('0', stdout);
	putc_unlocked('\n', stdout);
      }
    } else { // delete
      p2 = index(p1, '\n');
      strncpy(key, p1, p2-p1);
      key[p2-p1] = '\0';
      hash = compute_hash(key)%(n/2);
      if (a[hash]==NULL) continue;
      if (strcmp(a[hash]->key, key)==0) {
	a[hash] = a[hash]->next;
	continue;
      }
      Node *ptr = a[hash];
      Node *prev = ptr;
      while (ptr) {
	if (strcmp(ptr->key, key)==0) {
	  prev->next = ptr->next;
	  break;
	}
	prev = ptr;
	ptr = ptr->next;
      }
    }
  }
  return 0;
}

