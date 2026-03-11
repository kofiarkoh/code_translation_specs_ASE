#include<stdio.h>
#include<stdlib.h>
int MIN(int a,int b){return a<b?a:b;}
int ta[1010],to[2000010],nt[2000010],co[1000010],F[1010];
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
//呼び出す方
//sからtへの最大流を求める　ノード数v辺の数e　aからbへキャパcのフローが流れる
int maxf(int s,int t,int v,int e,int *a,int *b,int *c){
  int i,f,r;
  for(i=0;i<v;i++)ta[i]=-1;
  for(i=0;i<e;i++){
    nt[i  ]=ta[to[i+e]=a[i]];
    nt[i+e]=ta[to[i  ]=b[i]];
    co[ta[a[i]]=i  ]=c[i];
    co[ta[b[i]]=i+e]=0;
  }
  for(f=0;r=MF(s,t,e,1000000000);f+=r){
    for(i=0;i<v;i++)F[i]=0;
  }
  return f;
}
int main(){
  int n,s,t,i,j,l,u,v,f[100010]={};
  int a[200010],b[200010],c[200010],e=0,ans=0,d[100010];
  scanf("%d %d %d",&n,&s,&t);
  for(i=0;i<n;i++){
    scanf("%d %d",&u,&v);
    d[i]=u-v;
    if(abs(d[i])<=s||t<=abs(d[i])&&abs(d[i])<=s*2){
      ans++;//printf("%d %d",u,v);
      f[i]=1;
    }
  }//printf("%d\n",ans);
  for(i=0;i<n;i++){
    if(f[i]||d[i]>0)continue;
    for(j=0;j<n;j++){
      if(f[j]||d[j]<0)continue;
      l=abs(d[i]+d[j]);
      if(l<=s||t<=l&&l<=s*2){
	a[e  ]=i;
	b[e  ]=j;
	c[e++]=1;
      }
    }
  }
  for(i=0;i<n;i++){
    if(f[i])continue;
    if(d[i]<0){
      a[e  ]=n;
      b[e  ]=i;
      c[e++]=1;
    }
    else{
      a[e  ]=i;
      b[e  ]=n+1;
      c[e++]=1;
    }
  }
  //for(i=0;i<e;i++)printf("%d %d\n",a[i],b[i]);
  printf("%d\n",maxf(n,n+1,n+2,e,a,b,c)+ans);
  return 0;
}
  
      

