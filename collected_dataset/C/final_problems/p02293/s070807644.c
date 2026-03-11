#include<stdio.h>
#include<stdlib.h>
#include<math.h>

typedef long long int int64;

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))
#define ABS(a) ((a)>(0)?(a):-(a))

typedef struct vector{
  int64 x,y;
} vec;

int64 innerNorm(vec a,vec b){
  return a.x*b.x+a.y*b.y;
}

int64 norm(vec a){
  return a.x*a.x+a.y*a.y;
}

int isParallel(vec a,vec b){
  int64 in=innerNorm(a,b);
  int64 x=norm(a);
  int64 y=norm(b);
  return in*in==x*y;
}

int isOrthogonal(vec a,vec b){
  return innerNorm(a,b)==0;
}

void run(void){
  int query;
  scanf("%d",&query);
  while(query--){
    int a,b,c,d,p,q,r,s;
    scanf("%d%d%d%d%d%d%d%d",&a,&b,&c,&d,&p,&q,&r,&s);
    vec aa,bb;
    aa.x=c-a;
    aa.y=d-b;
    bb.x=r-p;
    bb.y=s-q;
    if(isParallel(aa,bb)){
      printf("2\n");
    } else if(isOrthogonal(aa,bb)){
      printf("1\n");
    } else {
      printf("0\n");
    }
  }
}

int main(void){
  run();
  return 0;
}

