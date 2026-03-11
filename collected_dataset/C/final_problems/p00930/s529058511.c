# include <stdio.h>

#define STSIZE 300001
#define TREESIZE 300001 * 4
#define MAX 300000

#define min(x, y) x < y ? x : y

int f_open = 0, f_close = 0;  //ひっくり返されたとき1になる
int tree[TREESIZE];  //木

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
        tree[n + i - 1] = sum;
        break;
      case ')':
        sum--;
        tree[n + i - 1] = sum;
        break;
    }
  }
  for(;i < n; ++i)
    tree[n + i - 1] = MAX;
  for(i = n - 1, j = n + i - 1; j > 0; j -= 2)
    tree[(j - 1) / 2] = min(tree[j], tree[j - 1]);
}

/*
//木の表示
void Print_tree(int tree[], int n){
  for(int i = 0; i < 2 * n - 1; i++)
    printf("%d ", tree[i]);
  printf("\n");
}
*/

//半閉区間[a, b)の最小値
int minimum(int a, int b, int k, int left, int right){
  if(right <= a || b <= left)
    return MAX;
  if(a <= left && right <= b)
    return tree[k];
  else{
    int vl = minimum(a, b, k * 2 + 1, left, (left + right) / 2);
    int vr = minimum(a, b, k * 2 + 2, (left + right) / 2, right);
    return min(vl, vr);
  }
}

//位置qの括弧をひっくり返す
void Flip(char st[], int q){
  switch (st[q-1]) {
    case '(':
      st[q-1] = ')';
      f_open = 1;
      break;
    case ')':
      st[q-1] = '(';
      f_close = 1;
      break;
  }
}

//ひっくり返したとき括弧の対応がとれている最も左の位置を返す
int Balance(char st[], int n){
  if(f_open){  //開きが閉じにひっくり返されたとき
    for(int i = 1; st[i - 1] != '\0'; ++i)
      if(st[i - 1] == ')'){  //一番左の閉じを開きにすればよい
        Flip(st, i);
        return i;
      }
  }
  else if(f_close){  //閉じが開きにひっくり返されたとき
    Make_tree(st, n);  //木の作成
    for(int i = 0; st[i] != '\0'; ++i){
      if(st[i] == '(' && tree[n + i - 1] >= 2){
        //printf("%d\n", minimum(i + 1, n, 0, 0, n));
        if(minimum(i + 1, n, 0, 0, n) > 1){
          Flip(st, i + 1);
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
  for(int i = 1; i <= q; i++){
    f_open = 0; f_close = 0;
    scanf("%d", &p);
    Flip(st, p);
    printf("%d\n", Balance(st, val));
    //Print_tree(tree, val);
  }
  return 0;
}

