#include<stdio.h>
int R=1,C=1,H[2000010],N[2000010];
//評価関数（いまはMIN）
int hyouka(int a,int b){
  if(C<b)return 1;
  if(C<a||b==0)return 0;
  return N[H[a]]<N[H[b]]?1:0;
}
//挿入関数
void hin(int a){
  int i=C++;
  for(N[H[0]=R]=a;hyouka(0,i/2);i/=2)H[i]=H[i/2];
  H[i]=R++;
}
//取り出す関数
int hout(){
  int rt=H[1],i,j=2,k=H[--C];
  for(i=1;hyouka(i,C);i=j)H[i]=H[j=i*2+1-hyouka(i*2,i*2+1)];
  H[j/2]=k;
  return rt;
}
int lb(int l,int r,int n,int *d){
  int m=(l+r)/2;
  if(l==r)return m;
  return n<=d[m]?lb(l,m,n,d):lb(m+1,r,n,d);
}
int main(){
  long long n,m,s=0,ans=0,e=0,i,b[200010]={};
  int c[200010],d[200010],a=0;
  scanf("%lld %lld",&n,&m);
  for(i=0;i<n;i++)scanf("%d",&d[i]);
  for(i=0;i<n;i++){
    s=(s+d[i]-1+m)%m;
    c[i]=s;
    hin((int)s);
  }
  //for(i=0;i<n;i++)printf("%d ",c[i]);printf("\n");
  for(i=0;i<n;i++)d[i]=N[hout()];
  d[n]=1e9+10;
  e=1;
  b[0]++;
  for(i=0;i<m-1;i++){
    //printf("%d\n",lb(0,n,c[i],d));
    b[lb(0,n,c[i],d)]++;
  }//printf("\n");
  a=0;
  //for(i=0;i<5;i++)printf("%lld ",b[i]);printf("\n");
  for(i=0;i<n;i++){
    b[lb(0,n,a,d)]--;
    //int j;
    //for(j=0;j<5;j++)printf("%lld ",b[j]);printf("\n");
    ans+=b[lb(0,n,a,d)];//printf("%lld %d\n",ans,lb(0,n,a,d));
    //printf("!%d %d\n",c[i+m],lb(0,n,c[i+m],d));
    if(i+m-1<n)b[lb(0,n,c[i+m-1],d)]++;
    //for(j=0;j<5;j++)printf("%lld ",b[j]);printf("\n");printf("\n");
    a=c[i];
  }
  printf("%lld\n",ans);
  return 0;
}
	   
