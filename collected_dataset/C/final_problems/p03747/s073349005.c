#include <stdio.h>
#include <stdlib.h>

int main(void)
{
  int n, l, t, i, j, k;
  int *x, *w;

  /*初期値入力*/
  scanf("%d %d %d", &n, &l, &t);
  /*動的配列を作成*/
  x = malloc(sizeof(int) * n);
  w = malloc(sizeof(int) * n);
  /*蟻の座標と進む向きを入力*/
  for (i=0;i<n;i++) {
    scanf("%d %d", &x[i], &w[i]);
    if (w[i] == 2) {
      w[i] = -1;
    }
  }

  /*処理*/
  /*時間のループ*/
  for (i=0;i<t;i++) {
    /*すべての蟻のループ　初めの向き編*/
    for (j=0;j<n;j++) {
      for (k=j;k<n;k++) {
        if (x[j] == x[k]) {
          w[j] = w[j] * -1;
          w[k] = w[k] * -1;
          break;
        }
      }
    }
    /*すべての蟻のループ　とりあえず衝突無視して移動編*/
    for (j=0;j<n;j++) {
      x[j] = (x[j] + w[j]) % l;
      if (x[j] == -1) {
        x[j] = l - 1;
      }
      }
    /*すべての蟻のループ　衝突の効果を考慮する編*/
    for (j=0;j<n;j++) {
      for (k=j;k<n;k++) {
        if ((x[j] - x[k] == 1 && w[j] == 1 && w[k] == -1) || (x[j] - x[k] == -1 && w[j] == -1 && w[k] == 1)) {
          w[j] = w[j] * -1;
          w[k] = w[k] * -1;
          x[j] = x[j] + w[j];
          x[k] = x[k] + w[k];
          break;
        } else if (x[j] - x[k] == l - 1 && w[j] == -1 && w[k] == 1) {
          w[j] = w[j] * -1;
          w[k] = w[k] * -1;
          x[j] = 0;
          x[k] = l - 1;
          break;
        } else if (x[j] - x[k] == -(l - 1) && w[j] == 1 && w[k] == -1) {
          w[j] = w[j] * -1;
          w[k] = w[k] * -1;
          x[j] = l - 1;
          x[k] = 0;
          break;
        }
      }
    }
  }

  /*出力*/
  for (i=0;i<n;i++) {
    printf("%d\n", x[i]);
  }
  free(x);
  free(w);
  return 0;
}
