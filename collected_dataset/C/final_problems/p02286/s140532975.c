#include<stdio.h>
#define NIL 0
#define MAX 200000
struct tree {
  int key;
  int priority;
  int left;
  int right;
};
int count = 1, root = NIL;
void Search(struct tree *p, int key);
int Lrotation(struct tree *table, int p);
int Rrotation(struct tree *table, int p);
void Insert(struct tree *p, int key, int priority);
void Delete(struct tree *p, int key);
void Dispm(struct tree *table, int p);
void Dispp(struct tree *table, int p);
int main(void)
{
  int i, n, key, priority;
  char command[7];
  struct tree table[MAX+1];
  scanf("%d", &n);
  for (i = 0; i < n; i++) {
    scanf("%*c%s", command);
    switch (command[0]) {
    case 'f':
      scanf("%d", &key);
      Search(table, key);
      break;
    case 'i':
      scanf("%d %d", &key, &priority);
      Insert(table, key, priority);
      break;
    case 'd':
      scanf("%d", &key);
      Delete(table, key);
      break;
    case 'p':
      Dispp(table, root);
      putchar('\n');
      Dispm(table, root);
      putchar('\n');
      break;
    }
  }
  return 0;
}
void Search(struct tree *p, int key)
{
  int u = root;
  while (u != NIL) {
    if (p[u].key == key) {
      printf("yes\n");
      return;
    } else if (p[u].key > key) {
      u = p[u].left;
    } else {
      u = p[u].right;
    }
  }
  printf("no\n");
}
int Lrotation(struct tree *table, int p)
{
  int q = table[p].right;
  table[p].right = table[q].left;
  table[q].left = p;
  return q;
}
int Rrotation(struct tree *table, int p)
{
  int q = table[p].left;
  table[p].left = table[q].right;
  table[q].right = p;
  return q;
}
void Insert(struct tree *p, int key, int priority)
{
  int parent = NIL, child, u = root, sp = 0, stack[MAX];
  while (u != NIL){
    parent = stack[sp++] = u;
    if (p[u].key == key) return;
    else if (p[u].key > key) u = p[u].left;
    else u = p[u].right;
  }
  p[count].key = key;
  p[count].priority = priority;
  p[count].left = p[count].right = NIL;
  child = u = count;
  if (parent == NIL) root = count++;
  else {
    if (p[parent].key > key) p[parent].left = count++;
    else p[parent].right = count++;
    while (sp > 1) {
      parent = stack[--sp];
      if (p[parent].left == child) {
        p[parent].left = u;
        if (p[u].priority > p[parent].priority) u = Rrotation(p, parent);
        else return;
      } else {
        p[parent].right = u;
        if (p[u].priority > p[parent].priority) u = Lrotation(p, parent);
        else return;
      }
      child = parent;
    }
    parent = stack[--sp];
    if (p[parent].left == child) {
      p[parent].left = u;
      if (p[u].priority > p[parent].priority) root = u = Rrotation(p, parent);
      else return;
    } else {
      p[parent].right = u;
      if (p[u].priority > p[parent].priority) root = u = Lrotation(p, parent);
      else return;
    }
  }
}
void Delete(struct tree *p, int key)
{
  int parent = NIL, u = root;
  while (u != NIL && p[u].key != key) {
    parent = u;
    if (p[u].key > key) {
      u = p[u].left;
    } else {
      u = p[u].right;
    }
  }
  if (u == NIL) return;
  if (parent == NIL && p[u].left != NIL && p[u].right != NIL) {
    if (p[p[u].left].priority > p[p[u].right].priority) {
      parent = root = Rrotation(p, u);
    } else {
      parent = root = Lrotation(p, u);
    }
  }
  while (1){
    if (p[u].left == NIL) {
      if (p[parent].left == u) {
        p[parent].left = p[u].right;
        if (parent == NIL) root = p[u].right;
      }
      else {
        p[parent].right = p[u].right;
        if (parent == NIL) root = p[u].right;
      }
      return;
    } else if (p[u].right == NIL) {
      if (p[parent].left == u) {
        p[parent].left = p[u].left;
        if (parent == NIL) root = p[u].left;
      }
      else {
        p[parent].right = p[u].left;
        if (parent == NIL) root = p[u].left;
      }
      return;
    } else {
      if (p[p[u].left].priority > p[p[u].right].priority) {
        if (u == p[parent].left) parent = p[parent].left = Rrotation(p, u);
        else parent = p[parent].right = Rrotation(p, u);
      } else {
        if (u == p[parent].left) parent = p[parent].left = Lrotation(p, u);
        else parent = p[parent].right = Lrotation(p, u);
      }
    }
  }
}
void Dispm(struct tree *table, int p)
{
  if (p == NIL) return;
  printf(" %d", table[p].key);
  Dispm(table, table[p].left);
  Dispm(table, table[p].right);
}
void Dispp(struct tree *table, int p)
{
  if (p == NIL) return;
  Dispp(table, table[p].left);
  printf(" %d", table[p].key);
  Dispp(table, table[p].right);
}

