#include<stdio.h>
int MIN(int a,int b){return a<b?a:b;}
int ta[1010],to[100010],nt[100010],co[100010],F[1010];
int MF(int s,int t,int e,int min){
  if(s==t||min==0)return min;
  int i,r;
  F[s]=1;
  for(i=ta[s];i+1;i=nt[i]){
    if(F[to[i]])continue;
    r=MF(to[i],t,e,MIN(min,co[i]));
    co[i]-=r;
    co[(i+e)%(2*e)]+=r;
    if(r)return r;
  }
  return 0;
}
int maxf(int s,int t,int v,int e,int *a,int *b,int *c){
  int i,f,r;
  for(i=0;i<=v;i++)ta[i]=-1;
  for(i=0;i<e;i++){
    nt[i  ]=ta[to[i+e]=a[i]];
    nt[i+e]=ta[to[i  ]=b[i]];
    co[ta[a[i]]=i  ]=c[i];
    co[ta[b[i]]=i+e]=0;
  }
  for(i=0;i<=v;i++)F[i]=0;
  for(f=0;r=MF(s,t,e,1000000000);f+=r){
    if(f>1)return f;
    for(i=0;i<=v;i++)F[i]=0;
  }
  return f;
}
int main(){
  int n,m,i,r=0,a[100010],b[100010],c[100010],s,t;
  scanf("%d %d",&n,&m);
  for(i=1;i<=n;i++){
    a[r]=i;
    b[r]=i+n;
    c[r++]=1;
  }
  while(m--){
    scanf("%d %d",&s,&t);
    a[r]=s+n;
    b[r]=0;
    c[r]=1;
    a[r+1]=t+n;
    b[r+1]=0;
    c[r+1]=1;
    printf("%s\n",maxf(1+n,0,n*2+1,r+2,a,b,c)>1?"Yes":"No");
    a[r]=s+n;
    b[r]=t;
    c[r++]=1;
    a[r]=t+n;
    b[r]=s;
    c[r++]=1;
  }
  return 0;
}