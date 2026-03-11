#include<stdio.h>
#include<stdlib.h>

typedef long long int int64;

int root(int *u,int x){
  if(u[x]==x) return x;
  return u[x]=root(u,u[x]);
}

int same(int *u,int x,int y){
  return root(u,x)==root(u,y);
}

void unite(int *u,int x,int y){
  x=root(u,x);
  y=root(u,y);
  if(x==y) return;
  u[x]=y;
}

void run(void){
  int n,m;
  scanf("%d%d",&n,&m);
  int *u=(int *)calloc(n+1,sizeof(int));
  int i;
  for(i=0;i<=n;i++) u[i]=i;
  for(i=0;i<m;i++){
    int a,b;
    scanf("%d%d",&a,&b);
    unite(u,a,b);
  }
  int a,b,c;
  a=b=c=0;
  for(i=1;i<=n;i++){
    if(same(u,1,i)){
      a++;
    } else if(same(u,2,i)){
      b++;
    } else {
      c++;
    }
  }
  if(a<=b){
    b+=c;
  } else {
    a+=c;
  }
  int64 ans=(int64)a*(a-1)/2+(int64)b*(b-1)/2-m;
  printf("%lld\n",ans);
}

int main(void){
  run();
  return 0;
}
