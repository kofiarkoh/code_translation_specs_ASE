#include<stdio.h>
#include<stdlib.h>

typedef long long int int64;

typedef struct line{
  int64 a,b;
} line;

int64 func(line l,int x){
  return l.a*x+l.b;;
}

void run(void){
  int n;
  int64 c;
  scanf("%d%lld",&n,&c);
  int *h=(int *)calloc(n,sizeof(int));
  int i;
  for(i=0;i<n;i++) scanf("%d",h+i);
  int64 *dp=(int64 *)calloc(n,sizeof(int64));
  line *q=(line *)calloc(n,sizeof(line));
  int front=0,last=0;
  q[last++]=(line){-2*h[0],(int64)h[0]*h[0]};
  for(i=1;i<n;i++){
    int x=h[i];
    while(front+1<last && func(q[front],x)>=func(q[front+1],x)) front++;
    dp[i]=(int64)x*x+c+func(q[front],x);
    line l={-2*x,dp[i]+(int64)x*x};
    while(front+1<last){
      line a=q[last-2];
      line b=q[last-1];
      if((l.a-b.a)*(a.b-l.b)<(l.a-a.a)*(b.a-l.b)) break;
      last--;
    }
    q[last++]=l;
  }
  printf("%lld\n",dp[n-1]);
}

int main(void){
  run();
  return 0;
}
