#include<stdio.h>
int R=1,C=1,H[2000010],N[2000010];
long long MAX(long long a,long long b){return a<b?b:a;}
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
int main(){
  int n,i,a;
  long long d[200010],e[200010],k,m,s=0,l=-1,r=1e18+10;
  scanf("%d %lld",&n,&k);
  for(i=0;i<n;i++){
    scanf("%d",&a);
    hin(a);
  }
  for(i=0;i<n;i++)d[i]=N[hout()];
  R=C=1;
  for(i=0;i<n;i++){
    scanf("%d",&a);
    hin(-a);
  }
  for(i=0;i<n;i++)e[i]=-N[hout()];
  //for(i=0;i<n;i++)printf("%lld %lld\n",d[i],e[i]);printf("\n");
  while(r-l-1){
    m=(l+r+1)/2;
    s=0;
    for(i=0;i<n;i++)s+=MAX((d[i]-m/e[i]),0);
    if(s<=k)r=m;
    else    l=m;
  }
  printf("%lld\n",r);
  return 0;
}
	
