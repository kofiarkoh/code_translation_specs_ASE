#include<stdio.h>
int ta[100010],nt[200010],to[200010],co[200010];
void mkls(int v,int e,int *a,int *b){
  int i;
  for(i=0;i<v;i++)ta[i]=-1;
  for(i=0;i<e;i++){
    nt[i  ]=ta[to[i+e]=a[i]];
    nt[i+e]=ta[to[i  ]=b[i]];
    ta[b[i]]=i+e;
    ta[a[i]]=i;
  }
}
void f(int v,int *c,int s){
  int i,q[100010],t=0,r;
  for(i=0;i<v;i++)c[i]=0;
  c[q[t=0]=s]=1;
  for(r=1;r-t;t++){
    for(i=ta[q[t]];i+1;i=nt[i]){
      if(c[to[i]]>0)continue;
      c[q[r++]=to[i]]=c[q[t]]+1;
    }
  }
}
int main(){
  int n,s,t,m,i,c[100010],d[100010],a[100010],b[100010];
  scanf("%d %d %d",&n,&s,&t);
  for(i=0;i<n-1;i++){
    scanf("%d %d",&a[i],&b[i]);
    a[i]--;
    b[i]--;
  }
  mkls(n,n-1,a,b);
  f(n,c,s-1);
  f(n,d,t-1);
  //for(i=0;i<n;i++)printf("%d %d %d\n",i+1,c[i],d[i]);
  for(i=0;i<n;i++){
    if(c[i]>d[i])continue;
    if(m<d[i])m=d[i];
  }
  printf("%d\n",m-2);
  return 0;
}
