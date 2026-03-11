#include <stdio.h>

int main() {

  int i,j,n,m;
  int a[100][100], b[100], c[100];

//c?????????
  for ( i = 0; i < n; i++) {

    c[i] = 0;

  }

//n,m??\???
  scanf("%d %d", &n, &m);


//a??\???
  for( i = 0; i < n; i++){
    for ( j = 0; j < m; j++) {

      scanf("%d ", &a[i][j]);

    }
  }


//b??\???
  for ( i = 0; i < m; i++) {

    scanf("%d", &b[i]);

  }



//????¨????
  for( i = 0; i < n; i++){
    for ( j = 0; j < m; j++) {

      c[i] += a[i][j] * b[j];

    }
  }

//c??????
  for( i = 0; i < n; i++){

      printf("%d\n", c[i]);

  }

return 0;
}