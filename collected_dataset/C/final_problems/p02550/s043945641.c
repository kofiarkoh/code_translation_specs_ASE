#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

#define ll long long

void swap (int *x, int *y) {
  int temp;

  temp = *x;
  *x = *y;
  *y = temp;

  return;
}


long int gcd(long int a, long int b){
  long int c;

  while (b > 0){
    c = a % b;
    a = b;
    b = c;
  }

  return a;
}

int upll(const void*a, const void*b){
  return*(ll*)a<*(ll*)b?-1:*(ll*)a>*(ll*)b?1:0;
}

int downll(const void*a, const void*b){
  return*(ll*)a<*(ll*)b?1:*(ll*)a>*(ll*)b?-1:0;
}

void sortup(ll*a,int n){
  qsort(a,n,sizeof(ll),upll);
}

void sortdown(ll*a,int n){
  qsort(a,n,sizeof(ll),downll);
}

long int modpow(long int a,long int n,long int mod){
  long int ans=1;
  while(n>0){
    if(n & 1){
      ans=ans*a%mod;
    }
    a=a*a%mod;
    n/=2;
  }
  return ans;
}

long int modinv(long int a,long int mod){
  return modpow(a,mod-2,mod);
}

int max(int a,int b){
  if(a<b){
    return b;
  }
  else{
    return a;
  }
}

int min(int a,int b){
  if(a<b){
    return a;
  }
  else{
    return b;
  }
}

void chmax(int *a,int b){
  if(*a < b){
    *a = b;
  }

  return;
}

void chmin(int *a,int b){
  if(*a > b){
    *a =b;
  }

  return;
}


int main(void){
  long int n;
  long int x,m;
  scanf("%ld%ld%ld",&n,&x,&m);
  if(m==1){
    printf("0\n");
    return 0;
  }
  int che[m+1];//鳩ノ巣原理判断基準の配列
  long int mod[m+1];//a[i]の余り
  for(int i=0;i<m+1;i++){
    che[i]=0;
    mod[i]=0;
  }
  long int ans=0;
  if(n<=m){  //n<=mの時はO(10^5)だからゴリゴリでできる.
    for(int i=0;i<n;i++){
      if(i==0){
        ans+=x%m;
      }else{
        ans+=x*x%m;
        x=x*x%m;
      }
    }  //ここまではok.
  }else{  //m<nの時,鳩ノ巣原理よりa[i]=a[j])i!=j)となるa[j]が存在する→a[j]の次はa[i+1],...,a[j]と繰り返す.
  long int loop=0,sta=0,fin=0,count=0,lcount=0;
    for(int i=0;i<m;i++){  //まずはa[i]=a[j](i!=j)となるものが存在しないところまでansを求める.
      if(i==0){
        x=x%m;
      }else{
        x=(x*x)%m;
      }
      che[x]++;
      mod[i]=x;
      fin=i-1;
      sta=x;  //ループは最初の値
      if(che[x]>1){
        break;
      }
    }
    for(int i=0;i<=fin;i++){
      if(mod[i]==sta){
        sta=i;
        break;
      }
    }
    for(int i=0;i<sta;i++){
      ans+=mod[i];
    }
    for(int i=sta;i<=fin;i++){
      loop+=mod[i];
      count++;
    }
    lcount=(n-(sta))/count;
    ans+=loop*lcount;
    n=(n-(sta))%lcount;
    for(int i=sta;i<sta+n;i++){
      ans+=mod[i];
    }
  }
  printf("%ld\n",ans);
  return 0;
}