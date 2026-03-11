#include<stdio.h>
long long R=1,C=1,H[200010],N[2000010];
int hyouka(int a,int b){
  if(C<b)return 1;
  if(C<a||b==0)return 0;
  return N[H[a]]<N[H[b]]?1:0;
}
void hin(long long a){
  int i=C++;
  for(N[H[0]=R]=a;hyouka(0,i/2);i/=2)H[i]=H[i/2];
  H[i]=R++;
}
int hout(){
  int rt=H[1],i,j=2,k=H[--C];
  for(i=1;hyouka(i,C);i=j)H[i]=H[j=i*2+1-hyouka(i*2,i*2+1)];
  H[j/2]=k;
  return rt;
}
int lb(int l,int r,long long n,long long *d){
  int m=(l+r)/2;
  if(l==r)return m;
  return n<=d[m]?lb(l,m,n,d):lb(m+1,r,n,d);
}
long long n,m,k,i,j,id,a,b,c=0,r=0,t,d[4000010],q[4000010],u[4000010]={};
int main(){
  int ans=0;
  int f[400010]={};
  scanf("%lld %lld %lld",&n,&m,&k);
  m+=2;
  for(i=0;i<k;i++){
    scanf("%lld %lld",&a,&b);
    hin(a*m+b);
  }
  for(i=1;i<n-1;i++)hin(i*m-2);//printf("%d\n",C);
  while(C-1)d[c++]=N[hout()];
  for(i=0;i<c;i++){
    if(d[i]/m==0)q[r++]=d[i];
  }//printf("%d\n",c);
  //for(i=0;i<c;i++)printf("%lld %lld\n",d[i]/m,d[i]%m);
  for(t=0;r-t;t++){//printf("%lld %lld %lld\n",q[t]/m,q[t]%m,u[t]);
    for(j=-1;j<2;j++){
      id=lb(0,c-1,q[t]+j+m,d);
      if(d[id]==q[t]+j+m){
	u[r]=u[t];
	q[r++]=q[t]+j+m;
      }
      else if(u[t]==0){
	u[r]=1;
	q[r++]=q[t]+j+m;
      }
      if(q[r-1]%m==0||q[r-1]/m==n-1){
	if(u[r-1]==0){
	  printf("0\n");
	  return 0;
	}
	ans=1;
      }
      if(q[r-1]%m==m-1)r--;
    }
  }
  printf("%d\n",ans?1:2);
  return 0;
}