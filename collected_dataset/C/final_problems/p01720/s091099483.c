#include<stdio.h>
int lb(int l,int r,int n,int *d){
  int m=(l+r)/2;
  if(l==r)return m;
  return n<=d[m]?lb(l,m,n,d):lb(m+1,r,n,d);
}
int ub(int l,int r,int n,int *d){
  int m=(l+r)/2;
  if(l==r)return m;
  return n<d[m]?ub(l,m,n,d):ub(m+1,r,n,d);
}
int cnt(int l,int r,int n,int *d){return ub(l,r,n,d)-lb(l,r,n,d);}
int n,m,s,g,r,t,a,b,l,ta[100010],to[600010],nt[600010],q[600010];
int main(){
  int c[2][100010]={};
  int i,f[100010]={};
  long long ans=0;
  scanf("%d %d %d %d",&n,&m,&s,&g);
  for(i=0;i<n;i++)ta[i+1]=-1;
  for(i=0;i<m;i++){
    scanf("%d %d",&a,&b);
    nt[i  ]=ta[to[i+m]=a];
    nt[i+m]=ta[to[i  ]=b];
    ta[a]=i  ;
    ta[b]=i+m;
  }
  q[t=0]=s;
  c[0][0]=0;
  f[s]=1;
  for(r=1;r-t;t++){
    if(q[t]==g)l=c[0][t];//printf("%d %d\n",q[t],c[0][q[t]]);
    for(i=ta[q[t]];i+1;i=nt[i]){//printf("%d ",c[0][q[t]]);
      if(f[to[i]])continue;//printf("%d ",c[0][q[t]]);
      c[0][r]=c[0][t]+1;//printf(":%d %d %d\n",q[t],c[0][r],to[i]);
      f[to[i]]=1;
      q[r++]=to[i];
    }
  }//printf("\n");
  m=r;//printf("%d %d\n",r,l);
  //for(i=0;i<r;i++)printf("%d ",c[0][i]);printf("\n");
  q[t=0]=g;
  f[g]=2;
  for(r=1;r-t;t++){
    ans+=cnt(0,m-1,l-2-c[1][t],c[0]);
    //printf("%d %d %d\n",a,c[1][t],q[t]);
    for(i=ta[q[t]];i+1;i=nt[i]){
      if(f[to[i]]-1)continue;
      c[1][r]=c[1][t]+1;
      f[to[i]]=2;
      q[r++]=to[i];
    }
  }
  printf("%lld\n",ans);
  return 0;
}
      