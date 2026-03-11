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
int a[1000][1000];
int cnt[1000];
int us[1000];
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
int main(void){
  int n,c=0,ans,j;
  scanf("%d",&n);
  FOR(n)FORJ(n-1)scanf("%d",&a[i][j]);
  for(ans=0;c*2<n*(n-1);ans++){
    int flg=1;
    FOR(n-1){j=a[i][cnt[i]]-1;
      if(cnt[i]==n-1||i>j)continue;
      if(us[i]+us[j])continue;
      if(i+1==a[j][cnt[j]]){
        us[i]++;
        us[j]++;
        cnt[i]++;
        cnt[j]++;
        c++;
        flg=0;
      }
    }
    if(flg){
      ans=-1;
      break;
    }
    FOR(n)us[i]=0;
  }
  PRN(ans);
  return 0;
}
