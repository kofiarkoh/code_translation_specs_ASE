#include<stdio.h>
#include<stdlib.h>
long long MIN(long long a,long long b){return a<b?a:b;}
int main(){
  int i,j,n,x[20],y[20],hr=0,wr=0,h[20]={0},w[20]={0};
  long long m,k,s,ans[20],z[20],p[20]={1};
  for(i=0;i<15;i++)p[i+1]=p[i]*3;
  for(i=0;i<20;i++)ans[i]=1e9;
  scanf("%d",&n);
  for(i=0;i<n;i++)scanf("%d %d %lld",&x[i],&y[i],&z[i]);
  for(i=0;i<p[n];i++){
    hr=wr=1;
    for(j=0;j<n;j++){
      if((i/p[j])%3==1)h[hr++]=y[j];
      if((i/p[j])%3==2)w[wr++]=x[j];
    }
    //for(j=0;j<hr;j++)printf("%d ",h[j]);
    //for(j=0;j<wr;j++)printf(" %d",w[j]);printf(":%d\n",hr+wr);
    if(hr+wr==n+2){
      ans[hr+wr]=0;
      continue;
    }
    s=0;
    for(j=0;j<n;j++){
      m=1e8;
      for(k=0;k<hr;k++)m=MIN(m,abs(y[j]-h[k]));
      for(k=0;k<wr;k++)m=MIN(m,abs(x[j]-w[k]));
      s+=m*z[j];
    }
    //printf("%lld\n",s);
    ans[hr+wr]=MIN(ans[hr+wr],s);
  }
  for(i=0;i<=n;i++)printf("%lld\n",ans[i+2]);
  return 0;
}
