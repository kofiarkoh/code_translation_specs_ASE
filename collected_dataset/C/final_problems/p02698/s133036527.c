#include<stdio.h>
int lis[200020],r=0,ans[200020],d[200020];
int ta[200020],nt[400040],to[400040];
int lb(int l,int r,int n,int *d){
  int m=(l+r)/2;
  if(l==r)return m;
  return n<=d[m]?lb(l,m,n,d):lb(m+1,r,n,d);
}
void f(int s,int t){//printf("%d %d\n",s,t);
  int rn,rp,i;
  ans[s]=r;
  for(i=ta[s];i+1;i=nt[i]){
    if(to[i]==t)continue;
    //int j;printf("lis %d:",s);
    //for(j=0;j<r;j++)printf("%d ",lis[j]);printf("\n");
    if(lis[r-1]<d[to[i]]){
      rn=-1;
      rp=r;
      lis[r++]=d[to[i]];
    }
    else{
      rp=lb(0,r,d[s],lis);
      rn=lis[rp];
      lis[rp]=d[to[i]];
    }
    f(to[i],s);
    if(rn>0)lis[rp]=rn;
    else    r--;
  }
}    
int main(){
  int n,i,a,b;
  scanf("%d",&n);
  for(i=0;i<n;i++)scanf("%d",&d[i+1]);
  for(i=0;i<n;i++)ta[i+1]=-1;
  for(i=0;i<n-1;i++){
    scanf("%d %d",&a,&b);
    nt[i    ]=ta[to[n-1+i]=a];
    nt[n-1+i]=ta[to[i    ]=b];
    ta[a]=i;
    ta[b]=n-1+i;
  }
  lis[r++]=d[1];
  f(1,0);
  for(i=0;i<n;i++)printf("%d\n",ans[i+1]);
  return 0;
}
