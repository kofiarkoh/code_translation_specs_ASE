#include<stdio.h>
int MIN(int a,int b){return a<b?a:b;}
int MAX(int a,int b){return a<b?b:a;}
int ta[100010],to[100010],co[100010],nt[100010],it[100010],l[100010];
int MF(int e,int s,int t,int min){
  if(s==t)return min;
  int i,r;
  for(;it[s]+1;it[s]=nt[it[s]]){
    if(co[it[s]]==0||l[s]>=l[to[it[s]]])continue;
    r=MF(e,to[it[s]],t,MIN(min,co[it[s]]));
    co[it[s]]-=r;
    co[(it[s]+e)%(2*e)]+=r;
    if(r)return r;
  }
  return 0;
}
//呼び出す方
//頂点v、辺eのグラフでsからgに流す
int maxf(int v,int e,int s,int g){
  int i,a=0,q[100010],t,r,c=3;
  while(1){
    for(i=t=0;i<v;i++)l[i]=0;
    for(r=l[q[0]=s]=1;r-t;t++){
      for(i=ta[q[t]];i+1;i=nt[i]){
	if(co[i]&&l[to[i]]==0)l[q[r++]=to[i]]=l[q[t]]+1;
      }
    }
    if(l[g]==0)return a;
    for(i=0;i<v;i++)it[i]=ta[i];
    while(r=MF(e,s,g,1e9))a+=r;
  }
}
void mklsb(int v,int e,int *a,int *b,int *c){
  int i;
  for(i=0;i<v;i++)ta[i]=-1;
  for(i=0;i<e;i++){
    nt[i  ]=ta[to[i+e]=a[i]];
    nt[i+e]=ta[to[i  ]=b[i]];
    co[ta[a[i]]=i  ]=c[i];      
    co[ta[b[i]]=i+e]=0;
  }
}
int main(){
  int n,m,i,j,t,r=0,a[100010],b[100010],c[100010],d;
  char s[100010];
  scanf("%d %d %s",&n,&m,s);
  for(i=0;s[i];i++){
    scanf("%d",&d);
    if(s[i]=='L'){
      a[r]=0;
      b[r]=i+1;
      c[r++]=0;
      a[r]=i+1;
      b[r]=n+1;
      c[r++]=d;
    }
    else{
      a[r]=0;
      b[r]=i+1;
      c[r++]=d;
      a[r]=i+1;
      b[r]=n+1;
      c[r++]=0;
    }
  }
  while(m--){
    scanf("%d %d %d",&i,&j,&t);
    a[r]=MIN(i,j);
    b[r]=MAX(i,j);
    c[r++]=t;
  }
  mklsb(n+2,r,a,b,c);
  printf("%d\n",maxf(n+2,r,0,n+1));
  return 0;
}
    
  

