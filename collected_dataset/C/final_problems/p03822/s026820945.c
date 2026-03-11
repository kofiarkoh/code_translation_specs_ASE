#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define PEOPLE 100001

int a[PEOPLE];
int **b;
// int b[PEOPLE][PEOPLE];
// int b[PEOPLE];
// int count[PEOPLE];
int num;
int maxnum;
int d[PEOPLE];
int count[PEOPLE];

int saiteikaisou(int n) {
  int max = -1;
  int i = 0;
  int temp;
  int c = 0;
  //  int b[d[n]+2];

/*  for (i = 0; i < d[n] + 2; i++) {
    b[i] = 0;
  }
  for (i = 2; i <= num; i++) {
    if (a[i] == n) {
      b[count] = i;
      count++;
    }
  }
  */
  if (b[n][i] == 0) {
    return 0;
  }

  while (b[n][i] != 0) {
    temp = saiteikaisou(b[n][i]);
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
  
  if(max+c+1>d[n]){
  return max + c + 1;
  }
  else {
    return d[n];
  }
}

int main() {
  int i, j;

  int ans;

  for (i = 0; i < PEOPLE; i++) {
    a[i] = 0;
    d[i] = 0;
    count[i] = 0;
    /*  for (j = 0; j < PEOPLE; j++) {
        b[i][j] = 0;
      }
      */
  }

  scanf("%d", &num);
  for (i = 2; i <= num; i++) {
    scanf("%d", &a[i]);
    d[a[i]]++;
  }

  for (i = 1; i <= num; i++) {
    if (maxnum < d[i]) {
      maxnum = d[i];
    }
  }

  b = (int **)malloc(sizeof(int *) * (num + 1));
  for (i = 0; i < num + 1; i++) {
    b[i] = (int *)calloc((d[i] + 2), sizeof(int));
  }

  for (i = 2; i <= num; i++) {
    b[a[i]][count[a[i]]] = i;
    //   printf("%d %d %d\n", a[i], count[a[i]], b[a[i]][count[a[i]]]);
    count[a[i]]++;
  }

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