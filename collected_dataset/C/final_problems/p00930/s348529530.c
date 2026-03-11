#include <stdio.h>

#define STSIZE 300001
#define TREESIZE 300001 * 4
#define MAX 300000

#define min(x, y) x < y ? x : y

//遅延評価セグメント木
typedef struct{
  int tree[TREESIZE], lazy[TREESIZE], lazyFlag[TREESIZE];
}segtree_t;

//プロトタイプ宣言
int Power2(int n);
void Make_tree(char st[], int n);
void update(int a, int b, int x, int k, int left, int right);
void lazyEvaluate(int k, int left, int right);
int minimum(int a, int b, int k, int left, int right);
int get_leaf(int pos, int k, int left, int right);
//void Print_tree(int n);
void Flip(char st[], int q, int val);
int Balance(char st[], int n);

int f_open = 0, f_close = 0;  //ひっくり返されたとき1になる
segtree_t segtree;

//nより大きい2の累乗
int Power2(int n){
  int value;
  for(value = 1; value < n; value *= 2)
    ;
  return value;
}

//木の作成
void Make_tree(char st[], int n){
  int sum = 0, i, j;

  for(i = 0; st[i] != '\0'; ++i){
    switch(st[i]){
      case '(':
        sum++;
        segtree.tree[n + i - 1] = sum;
        break;
      case ')':
        sum--;
        segtree.tree[n + i - 1] = sum;
        break;
    }
  }
  for(;i < n; ++i)
    segtree.tree[n + i - 1] = MAX;
  for(i = n - 1, j = n + i - 1; j > 0; j -= 2)
    segtree.tree[(j - 1) / 2] = min(segtree.tree[j], segtree.tree[j - 1]);
}

//値の更新
void update(int a, int b, int x, int k, int left, int right){
  lazyEvaluate(k, left, right);
  if(right <= a || b <= left)
    return;
  if(a <= left && right <= b){
    segtree.lazy[k] += x;
    segtree.lazyFlag[k] = 1;
    lazyEvaluate(k, left, right);
  }
  else{
    update(a, b, x, k * 2 + 1, left, (left + right) / 2);
    update(a, b, x, k * 2 + 2, (left + right) / 2, right);
    segtree.tree[k] = min(segtree.tree[k * 2 + 1], segtree.tree[k * 2 + 2]);
  }
}

//遅延評価
void lazyEvaluate(int k, int left, int right){
  if(segtree.lazyFlag[k]){
    segtree.tree[k] += segtree.lazy[k];
    if(right - left > 1){  //kが葉ではないとき
      segtree.lazy[k * 2 + 1] += segtree.lazy[k];
      segtree.lazy[k * 2 + 2] += segtree.lazy[k];
      segtree.lazyFlag[k * 2 + 1] = segtree.lazyFlag[k * 2 + 2] = 1;
    }
    segtree.lazy[k] = 0;
    segtree.lazyFlag[k] = 0;
  }
}

//半閉区間[a, b)の最小値
int minimum(int a, int b, int k, int left, int right){
  lazyEvaluate(k, left, right);
  if(right <= a || b <= left)
    return MAX;
  if(a <= left && right <= b)
    return segtree.tree[k];
  else{
    int vl = minimum(a, b, k * 2 + 1, left, (left + right) / 2);
    int vr = minimum(a, b, k * 2 + 2, (left + right) / 2, right);
    return min(vl, vr);
  }
}

//葉ノードの値を取得
int get_leaf(int pos, int k, int left, int right){
  lazyEvaluate(k, left, right);
  if(pos == left)
    return segtree.tree[k];
  if(pos < (left + right) / 2)
    return get_leaf(pos, k * 2 + 1, left, (left + right) / 2);
  else
    return get_leaf(pos, k * 2 + 2, (left + right) / 2, right);
}

/*
//木の表示
void Print_tree(int n){
  for(int i = 0; i < 2 * n - 1; i++)
    printf("%d ", segtree.tree[i]);
  printf("\n");
}
*/

//位置qの括弧をひっくり返す
void Flip(char st[], int q, int n){
  switch (st[q-1]) {
    case '(':
      st[q-1] = ')';
      f_open = 1;
      update(q - 1, n, -2, 0, 0, n);
      break;
    case ')':
      st[q-1] = '(';
      f_close = 1;
      update(q - 1, n, 2, 0, 0, n);
      break;
  }
}

//ひっくり返したとき括弧の対応がとれている最も左の位置を返す
int Balance(char st[], int n){
  if(f_open){  //開きが閉じにひっくり返されたとき
    for(int i = 1; st[i - 1] != '\0'; ++i)
      if(st[i - 1] == ')'){  //一番左の閉じを開きにすればよい
        Flip(st, i, n);
        return i;
      }
  }
  else if(f_close){  //閉じが開きにひっくり返されたとき
    for(int i = 0; st[i] != '\0'; ++i){
      if(st[i] == '(' && get_leaf(i, 0, 0, n) >= 2){
        //update(i + 1, n, 0, 0, 0, n);
        //printf("%d\n", minimum(i + 1, n, 0, 0, n));
        if(minimum(i + 1, n, 0, 0, n) > 1){
          Flip(st, i + 1, n);
          return i + 1;
        }
      }
    }
  }
  else
    return -1;
}

int main(){
  char st[STSIZE];
  int p, q, n;
  int val;

  scanf("%d%d", &n, &q);
  val = Power2(n);
  scanf("%s", st);
  Make_tree(st, val);
  //Print_tree(val);
  for(int i = 0; i < 2 * val - 1; ++i)
    segtree.lazy[i] = 0;
  for(int i = 1; i <= q; i++){
    f_open = 0; f_close = 0;
    scanf("%d", &p);
    Flip(st, p, val);
    printf("%d\n", Balance(st, val));
    //Print_tree(val);
  }
  return 0;
}
