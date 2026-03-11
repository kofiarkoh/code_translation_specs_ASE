#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>
#define FOR(n) for(int i=0;i<n;i++)
#define FORJ(n) for(int j=0;j<n;j++)
#define PRN(n) printf("%d\n",n)
#define PRF(n) printf("%lf\n",n)
#define PRL(n) printf("%lld\n",n)
#define PRS(s) printf("%s\n",s)
#define PRC(c) printf("%c",c)
#define mod 1000000007
typedef long long int ll;
int u(const void *a, const void *b){
    return *(ll*)a-*(ll*)b;
}
int d(const void *a, const void *b){
    return *(ll*)b-*(ll*)a;
}
int z(int a){
  if(a<1)a*=-1;
  return a;
}
int min(int a,int b){
  if(a>b)return b;
  return a;
}
int max(int a,int b){
  if(a>b)return a;
  return b;
}
int gcd(int a,int b){
  if(!b)return a;
  return gcd(b,a%b);
}
int kt(ll a){
  int sum=0;
  while(a){
    a/=10;
    sum++;
  }
  return sum;
}
int ks(ll a){
  int sum=0;
  while(a){
    sum+=a%10;
    a/=10;
  }
  return sum;
}
int main(void){
  int n;
  scanf("%d",&n);
  int a[n];
  FOR(n)scanf("%d",&a[i]);
  qsort(a,n,sizeof(int),u);
  int ans=0;
  int p[a[n-1]],cnt=0;
  for(int i=1;i<=(int)sqrt(a[n-1]);i++){
    if(a[n-1]%i<1){
      p[cnt]=i;
      p[cnt+1]=a[n-1]/i;
      cnt+=2;
    }
  }
  qsort(p,cnt,sizeof(int),u);
  FOR(n-1){
    int x=0;
    while(p[x]<a[i])x++;
    ans+=p[x]-a[i];
  }
  PRN(ans);
  return 0;
}

