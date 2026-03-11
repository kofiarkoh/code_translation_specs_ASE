#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <assert.h>
#include <time.h>

/////////////////////////////////////////////////////////////////
// Red Black Tree
/////////////////////////////////////////////////////////////////
// totemo sankou nishita: http://algoogle.hadrori.jp/algorithm/rbtree.html

typedef int key_t;

typedef struct node {
  key_t key;
  int red;
  struct node *par, *ch[2];
} node_t;

typedef node_t* node_ptr;

node_ptr node_create(key_t key) {
  node_ptr p = (node_ptr)malloc(sizeof(node_t));
  p->key = key;
  p->red = 1;
  p->par = p->ch[0] = p->ch[1] = NULL;
  return p;
}

unsigned rb_sz;
node_ptr rb_root, rb_nil;

void rb_init(){ 
  rb_sz = 0;
  rb_nil = node_create(-1);
  rb_nil->key = -1;
  rb_nil->red = 0;
  rb_root = node_create(-1);

  rb_root
  = rb_nil->par = rb_nil->ch[0] = rb_nil->ch[1]
  = rb_nil;
}

unsigned rb_size() { return rb_sz; }

void rb_print_node(node_ptr v, int depth, int lr) {
  int i;

  if(v == rb_nil)
    return;

  rb_print_node(v->ch[1], depth+1, 1);

  for(i = 0; i < depth; i++)
    printf("  ");

  if(!lr)
    printf("--");
  else if(lr == 1)
    printf("???");
  else
    printf("??¬");

  if(v->red)
    printf("\x1b[31m");

  printf("%d\n", v->key);
  printf("\x1b[0m");
  rb_print_node(v->ch[0], depth + 1, 2);
}

void rb_print(){
  assert(rb_root);
  rb_print_node(rb_root, 0, 0);
}

void rb_rotate(node_ptr x, int d) {
  int e = d^1;
  node_ptr y = x->ch[e];
  x->ch[e] = y->ch[d];
  if(y->ch[d] != rb_nil) y->ch[d]->par = x;
  y->par = x->par;
  if(x->par == rb_nil) rb_root = y;
  else if(x == x->par->ch[d]) x->par->ch[d] = y;
  else x->par->ch[e] = y;
  y->ch[d] = x;
  x->par = y;
}

node_ptr rb_find(key_t key) {
  node_ptr x = rb_root;
  while(x != rb_nil) {
    if(key != x->key) x = x->ch[key > x->key];
    else return x;
  }
  return x;
}

void rb_insert_update(node_ptr z) {
  node_ptr y;
  int d, e;
  while(z->par->red) {
    d = z->par == z->par->par->ch[0] ? 0 : 1;
    e = d^1;
    y = z->par->par->ch[e];
    if(y->red) {
      z->par->red = 0;
      y->red = 0;
      z->par->par->red = 1;
      z = z->par->par;
    }
    else if(z == z->par->ch[e]) {
      z = z->par;
      rb_rotate(z,d);
    }
    else {
      z->par->red = 0;
      z->par->par->red = 1;
      rb_rotate(z->par->par,e);
    }
  }
  rb_root->red = 0;
}

node_ptr rb_insert(key_t key) {
  node_ptr x = rb_root, y = rb_nil;
  node_ptr z;
  while(x != rb_nil) {
    y = x;
    if(key != x->key) x = x->ch[key > x->key];
    else return x;
  }
  rb_sz++;
  z = node_create(key);
  z->par = y;
  if(y == rb_nil) rb_root = z;
  else y->ch[z->key >= y->key] = z;
  z->ch[0] = z->ch[1] = rb_nil;
  rb_insert_update(z);
  rb_nil->par = rb_nil->ch[0] = rb_nil->ch[1] = rb_nil;
  return z;
}

void rb_transplant(node_ptr u, node_ptr v) {
  if(u->par == rb_nil) rb_root = v;
  else if(u == u->par->ch[0]) u->par->ch[0] = v;
  else u->par->ch[1] = v;
  v->par = u->par;
}

node_ptr rb_minimum(node_ptr x) {
  while(x->ch[0] != rb_nil) x = x->ch[0];
  return x;
}

void rb_erase_update(node_ptr x) {
  node_ptr w = rb_nil;
  int d, e;
  while(x != rb_root && !x->red) {
    d = x == x->par->ch[0]? 0: 1, e = d^1;
    w = x->par->ch[e];
    if(w->red) {
      w->red = 0;
      x->par->red = 1;
      rb_rotate(x->par,d);
      w = x->par->ch[e];
    }
    else if(!w->ch[d]->red && !w->ch[e]->red) {
      w->red = 1;
      x = x->par;
    }
    else if(!w->ch[e]->red) {
      w->ch[d]->red = 0;
      w->red = 1;
      rb_rotate(w,e);
      w = x->par->ch[e];
    }
    else {
      w->red = x->par->red;
      x->par->red = 0;
      w->ch[e]->red = 0;
      rb_rotate(x->par,d);
      x = rb_root;
    }
  }
  x->red = 0;
}

void rb_erase_node(node_ptr z) {
  node_ptr y, x;
  int prevy;
  if(z == rb_nil) return;
  rb_sz--;
  y = z;
  prevy = y->red;
  if(z->ch[0] == rb_nil) {
    x = z->ch[1];
    rb_transplant(z,z->ch[1]);
  }
  else if(z->ch[1] == rb_nil) {
    x = z->ch[0];
    rb_transplant(z,z->ch[0]);
  }
  else {
    y = rb_minimum(z->ch[1]);
    prevy = y->red;
    x = y->ch[1];
    if(y->par == z) x->par = y;
    else {
      rb_transplant(y,y->ch[1]);
      y->ch[1] = z->ch[1];
      y->ch[1]->par = y;
    }
    rb_transplant(z,y);
    y->ch[0] = z->ch[0];
    y->ch[0]->par = y;
    y->red = z->red;
  }
  rb_nil->par = rb_nil->ch[0] = rb_nil->ch[1] = rb_nil;
  if(!prevy) rb_erase_update(x);
  free(z);
  rb_nil->par = rb_nil->ch[0] = rb_nil->ch[1] = rb_nil;
}

void rb_erase(key_t key) {
  rb_erase_node(rb_find(key));
}

node_ptr rb_successor(node_ptr x) {
  node_ptr y;
  if(x->ch[1] != rb_nil)
    return rb_minimum(x->ch[1]);
  y = x->par;
  while(y != rb_nil && x == y->ch[1]) {
    x = y;
    y = y->par;
  }
  return y;
}

node_ptr rb_binary_search(key_t key, int upper_bound) {
  node_ptr x = rb_root;
  while(x != rb_nil) {
    if(key != x->key) {
      if(x->ch[key > x->key] == rb_nil) {
        if(key > x->key)
          return rb_successor(x);
        else return x;
      }
      else {
        x = x->ch[key > x->key];
      }
    }
    else return upper_bound ? rb_successor(x) : x;
  }
  return x;
}

node_ptr rb_lower_bound(key_t key) {
  return rb_binary_search(key, 0);
}

node_ptr rb_upper_bound(key_t key) {
  return rb_binary_search(key, 1);
}

/////////////////////////////////////////////////////////////////
// End of Red Black Tree
/////////////////////////////////////////////////////////////////

enum {
  VTop = 0,
  HLeft = 1,
  VBottom = 2
};
  
typedef struct P_ {
  int x, y;
  int type;
  int oppo_x;
} P;

int max(int x, int y) {
  return x < y ? y : x;
}

int min(int x, int y) {
  return x < y ? x : y;
}

int compare_P(const void *a_, const void *b_) {
  const P *a = (const P *)a_;
  const P *b = (const P *)b_;
  if(a->y != b->y) return a->y - b->y;
  if(a->type != b->type) return a->type - b->type;
  return a->x - b->x;
}
struct Line { P ps[2]; };

void swap(int* a, int* b) {
  int tmp = *a;
  *a = *b;
  *b = tmp;
}

int main() {
  
  #if 0
  /*
    Red-black tree test code
  */
  int i;
  rb_init();
  srand((unsigned)(time(NULL)));
  for(i=0; i<100; i++)
    rb_insert(rand());
  rb_print();

  printf("minimum: %d\n", rb_minimum(rb_root)->key);

  while(~scanf("%d", &i)) {
    printf("lower_bound(%d): %d\n", i, rb_lower_bound(i)->key);
    printf("upper_bound(%d): %d\n", i, rb_upper_bound(i)->key);
    printf("successor(lower_bound(%d)): %d\n", i, rb_successor(rb_lower_bound(i))->key);
//    rb_erase(i);
//    rb_print();
//    printf("deleted: %d\n", i);
  }

  return 0;
}
  #else

  int N;
  P pts[100001];
  int pcnt = 0;
  int x1, y1, x2, y2;
  int type1, type2;
  int ret;
  int cnt;
  int i;
  node_ptr iter1 = NULL, iter2 = NULL;

  scanf("%d", &N);
  for(i = 0; i < N; i++) {
    scanf("%d%d%d%d", &x1, &y1, &x2, &y2);
    if(x1 > x2) swap(&x1, &x2);
    if(y1 > y2) swap(&y1, &y2);
    if(x1 == x2) {
      type1 = VTop, type2 = VBottom;
      pts[pcnt++] = (P){x1, y1, type1, -1};
      pts[pcnt++] = (P){x2, y2, type2, -1};
    }
    else {
      type1 = HLeft;
      pts[pcnt++] = (P){x1, y1, type1, x2};
    }
  }

  qsort(pts, pcnt, sizeof(P), (int (*)(const void *, const void *))compare_P);

  ret = 0;
  rb_init();

  for(i = 0; i < pcnt; i++) {
    if(pts[i].type == VTop) {
      rb_insert(pts[i].x);
    }
    if(pts[i].type == VBottom) {
      rb_erase(pts[i].x);
    }
    if(pts[i].type == HLeft) {
      x1 = pts[i].x, x2 = pts[i].oppo_x;
      iter1 = rb_lower_bound(x1);
      iter2 = rb_upper_bound(x2);
 
      cnt = 0;
      while(iter1 != iter2) {
        iter1 = rb_successor(iter1); cnt++;
      }
      ret += cnt;
    }
  }
    
  printf("%d\n", ret);
    
  return 0;
}
#endif