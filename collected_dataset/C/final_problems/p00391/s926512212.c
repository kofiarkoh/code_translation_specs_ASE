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
int main(void){
  int w,h;
  scanf("%d %d",&w,&h);
  int a[w],b[h],c[h];
  FOR(w)scanf("%d",&a[i]);
  FOR(h)scanf("%d",&b[i]);
  qsort(a,w,sizeof(int),d);
  qsort(b,h,sizeof(int),d);
  FOR(h)c[i]=b[i];
  FOR(w){
    FORJ(a[i]){
      if(!b[j]){
        PRN(0);
        return 0;
      }
      b[j]--;
    }
    qsort(b,h,sizeof(int),d);
  }
  FOR(h){
    FORJ(c[i]){
      if(!a[j]){
        PRN(0);
        return 0;
      }
      a[j]--;
    }
    qsort(a,w,sizeof(int),d);
  }
  PRN(1);
  return 0;
}

