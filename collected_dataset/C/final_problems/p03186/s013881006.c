#include<stdio.h>
#include<stdlib.h>
#include<math.h>

typedef long long int int64;

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))
#define ABS(a) ((a)>(0)?(a):-(a))

void run(void){
  int a,b,c;
  scanf("%d%d%d",&a,&b,&c);
  int ans=b+MIN(c,a+b+1);
  printf("%d\n",ans);
}

int main(void){
  run();
  return 0;
}