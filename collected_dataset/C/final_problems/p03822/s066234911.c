#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define PEOPLE 100001  //最大出場者数

int a[PEOPLE];
int **b;        //各人の勝った相手一覧
int num;        //出場者数
int d[PEOPLE];  //各人の勝数
int count[PEOPLE];
/*
void mergesort(int *array, int left, int right) {
  if(left+1<right){}
  int mid = (left+rught) / 2;
  mergesort(array, left, mid);
  mergesort(array, mid , right);
  merge(array, left, mid, right);
 }
 */
int int_cmp(const int *a, const int *b) {
  if (*a > *b)
    return (-1);
  else if (*a < *b)
    return (1);
  return (0);
}

int saiteikaisou(int n) {  //再帰を使って最小深さを求める
  int max =
      -1;  //敗者の中でその段階までの階層数が最大のもの→maxの階層数を持つ敗者を最後に対戦させる。
  int i = 0;
  int *temp;
  temp = (int *)malloc(sizeof(int) * d[n]);
  int c = 0;  // maxの階層数を持つ敗者の数

  if (b[n][i] == 0) {
    return 0;
  }

  while (b[n][i] != 0) {
    temp[i] = saiteikaisou(b[n][i]);
    i++;
  }
  qsort(temp, d[n], sizeof(int), (int(*)(const void*, const void*))int_cmp);

  for (i = 0; i < d[n]; i++) {
    if (max < i + temp[i] + 1) {
      max = i + temp[i] + 1;
    }
  }

  return max;
}

int main() {
  int i, j;

  int ans;

  for (i = 0; i < PEOPLE; i++) {  //初期化
    a[i] = 0;
    d[i] = 0;
    count[i] = 0;
  }

  scanf("%d", &num);
  for (i = 2; i <= num; i++) {
    scanf("%d", &a[i]);
    d[a[i]]++;
  }

  b = (int **)malloc(sizeof(int *) * (num + 1));  //各人の
  for (i = 0; i < num + 1; i++) {
    b[i] = (int *)calloc((d[i] + 2),
                         sizeof(int));  //勝った相手の番号を入れる入れ物
  }

  for (i = 2; i <= num; i++) {  //勝った相手のリストを作成
    b[a[i]][count[a[i]]] = i;
    count[a[i]]++;
  }

  ans = saiteikaisou(1);
  printf("%d", ans);

  return 0;
}
