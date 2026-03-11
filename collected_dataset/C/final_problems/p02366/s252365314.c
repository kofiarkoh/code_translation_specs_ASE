#include<stdio.h>
int MIN(int a,int b){return a<b?a:b;}
int r,ord[100010],low[100010];
void cdfs(int s,int *ta,int *to,int *nt,int *p){//printf("s%d %d\n",s,r);
  int i;
  ord[s]=low[s]=r++;
  for(i=ta[s];i+1;i=nt[i]){
    if(p[to[i]]==-1){
      p[to[i]]=s;
      cdfs(to[i],ta,to,nt,p);
      low[s]=MIN(low[s],low[to[i]]);
    }
    else if(to[i]-p[s])low[s]=MIN(low[s],ord[to[i]]);
  }
}
void arpo(int v,int e,int *a,int *b){
  int ta[100010],to[100010],nt[100010],p[100010],ans=0,c=0,i,j,f[100010]={0};
  for(i=0;i<v;i++)ta[i]=p[i]=-1;
  for(p[0]=i=0;i<e;i++){
    nt[i  ]=ta[to[i+e]=a[i]];
    nt[i+e]=ta[to[i  ]=b[i]];
    ta[b[ta[a[i]]=i]]=i+e;
  }
  cdfs(0,ta,to,nt,p);
  for(i=1;i<v;i++){
    if(p[i]){
      if(ord[p[i]]<=low[i])f[p[i]]=1;
    }
    else c++;
  }
  if(c>1)f[0]=1;
  for(i=0;i<v;i++){
    if(f[i])printf("%d\n",i);
  }
}
int main(){
  int v,e,m,i,a[100010],b[100010],c[100010],d[100010];
  scanf("%d %d",&v,&e);
  for(i=0;i<e;i++)scanf("%d %d",&a[i],&b[i]);
  arpo(v,e,a,b);
  return 0;
}