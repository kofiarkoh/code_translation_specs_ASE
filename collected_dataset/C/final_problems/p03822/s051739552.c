#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define PEOPLE 10001

int a[PEOPLE];
// int b[PEOPLE][PEOPLE];
//int b[PEOPLE];
// int count[PEOPLE];
int num;

int saiteikaisou(int n) {
  int max = -1;
  int i = 0;
  int temp;
  int c = 0;
  int count = 0;
  int b[PEOPLE];

  for (i = 0; i < PEOPLE; i++) {
    b[i] = 0;
  }
  for (i = 2; i <= num; i++) {
    if (a[i] == n) {
      b[count] = i;
      count++;
    }
  }
  i = 0;
  if (b[i] == 0) {
    return 0;
  }

  while (b[i] != 0) {
    temp = saiteikaisou(b[i]);
    // printf("%d\n", temp);
    if (max < temp) {
      max = temp;
      c = 0;
    } else if (max == temp) {
      c++;
    } else {
    }
    i++;
  }

  return max + c + 1;
}

int main() {
  int i, j;

  int ans;
  for (i = 0; i < PEOPLE; i++) {
    a[i] = 0;
  //  count[i] = 0;
    /*  for (j = 0; j < PEOPLE; j++) {
        b[i][j] = 0;
      }
      */
  }

  scanf("%d", &num);
  for (i = 2; i <= num; i++) {
    scanf("%d", &a[i]);
  }

  /*
    for (i = 2; i <= num; i++) {

      b[a[i]][count[a[i]]] = i;
           //   printf("%d %d %d\n", a[i], count[a[i]], b[a[i]][count[a[i]]]);
      count[a[i]]++;

    }
    */

  ans = saiteikaisou(1);
  printf("%d", ans);

  return 0;
}

/*
typedef struct Node {
  int value;
  int *left;
  int *right;
};

int main() {
  int i, j;
  int a[PEOPLE];
  int b[PEOPLE];
  int c[PEOPLE];
  int num;
  int flag = 0;

  int kaisou;

  for (i = 0; i < PEOPLE; i++) {
    a[i] = 0;
    b[i] = 0;
    c[i] = 0;
  }

  sacnf("%d", &num);
  for (i = 2; i <= num; i++) {
    scanf("%d", &a[i]);
    b[a[i]]++;
  }

  for (i = 2; i <= num; i++)

    for (i = 2; i <= num; i++) {
      c[b[i]]++;
    }

  for (i = num; i > 1;i--){
    if (c[i] != 0 && flag == 0) {
      kaisou = i+1;
      flag = 1;
    }


  } return 0;
}
*/