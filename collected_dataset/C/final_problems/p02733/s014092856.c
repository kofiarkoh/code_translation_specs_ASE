#include <stdio.h>
#include <math.h>

int main(void){

  int h,w,k;
  scanf("%d %d %d", &h, &w, &k);
  char chocolate[h][w];
  for (int i = 0; i < h; i++) {
    scanf("%s", chocolate[i]);
  }

  int white[h];
  int h_cut[h-1];
  for (int i = 0; i < h-1; i++) {
    h_cut[i] = 0;
  }
  long cuts;
  long min_cuts = (h-1)*(w-1);

  int flag;

  int width;

  for (int i = 0; i < pow(2,h-1); i++) {
    cuts = 0;
    for (int j = 0; j < h-1; j++) {
      cuts += h_cut[j];
    }
    for (int j = 0; j < h; j++) {
      white[j] = 0;
    }
    width = 0;
    for (int j = 0; j < w; j++) {
      width++;
      if (h_cut[0] == 0) {
        white[0] = chocolate[0][j] - '0';
      } else {
        white[0] += chocolate[0][j] - '0';
      }
      for (int l = 1; l < h-1; l++) {
        if (h_cut[l] == 0) {
          if (h_cut[l-1] == 0) {
            white[l] = white[l-1];
            white[l] += chocolate[l][j] - '0';
          } else {
            white[l] = chocolate[l][j] - '0';
          }
        } else {
          if (h_cut[l-1] == 0) {
            white[l] += white[l-1];
          }
          white[l] += chocolate[l][j] - '0';
        }
      }
      if (h_cut[h-2] == 0) {
        white[h-1] += white[h-2];
      }
      white[h-1] += chocolate[h-1][j] - '0';

      flag = 0;
      for (int l = 0; l < h; l++) {
        if (white[l] > k) {
          flag = 1;
          break;
        }
      }
      if (flag == 1 && width > 0) {
        cuts++;
        width = 0;
        for (int l = 0; l < h; l++) {
          white[l] = 0;
        }
        j--;
      }
      if (flag == 1 && width == 1) {
        cuts = (h-1)*(w-1);
        break;
      }
    }

    //printf("%d\n", cuts);
    if (cuts < min_cuts) {
      min_cuts = cuts;
    }

    h_cut[0]++;
    for (int j = 0; j < h-1; j++) {
      if (h_cut[j] == 2) {
        h_cut[j+1]++;
        h_cut[j] = 0;
      } else {
        break;
      }
    }
  }

  printf("%ld\n", min_cuts);

  return 0;
}