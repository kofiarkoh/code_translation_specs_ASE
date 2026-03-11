/*
  A - キャンディーと2人の子供 / Fighting over Candies
  <http://abc047.contest.atcoder.jp/tasks/abc047_a>
*/

#include <stdio.h>
#include <stdlib.h>

int main(int argc, char const *argv[]) {
  int a,b,c;

  scanf("%d %d %d", &a, &b, &c);

  if( (a<1 || a>100) || (b<1 || b>100) || (c<1 || c>100) ){
    fprintf(stderr, "INPUT ERROR!\n");
    return -1;
  }

  if( (a+b==c) || (a+c==b) || (b+c==a) ){
    printf("Yes\n");
  }else{
    printf("No\n");
  }

  return 0;
}
