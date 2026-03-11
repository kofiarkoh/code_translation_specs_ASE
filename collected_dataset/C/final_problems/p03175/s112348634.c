#include<stdio.h>
long long d[100010][2];
int ta[100010],to[200010],nt[200010],M=1e9+7,u[100010];
void f(int s){
  d[s][0]=d[s][1]=1;
  u[s]=1;
  int i;
  for(i=ta[s];i+1;i=nt[i]){
    if(u[to[i]])continue;
    f(to[i]);
    d[s][0]=(d[s][0]*(d[to[i]][0]+d[to[i]][1])%M)%M;
    d[s][1]=(d[s][1]* d[to[i]][0]               )%M;
  }//printf("%d %lld %lld\n",s,d[s][0],d[s][1]);
  return ;
}
int main(){
  int n,a,b,i;
  scanf("%d",&n);
  for(i=0;i<n;i++)ta[i+1]=-1;
  for(i=0;i<n-1;i++){
    scanf("%d %d",&a,&b);
    nt[i]=ta[to[i+n-1]=a];
    ta[a]=i;
    nt[i+n-1]=ta[to[i]=b];
    ta[b]=i+n-1;
  }
  f(1);
  printf("%lld\n",(d[1][0]+d[1][1])%M);
  return 0;
}
