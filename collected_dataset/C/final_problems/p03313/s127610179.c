//Fast Zeta conversion
//cf. http://d.hatena.ne.jp/todo314/20120614/1339695202

#include<stdio.h>

long long max(long long a,long long b){if(a>b){return a;}return b;}

long long pow(long long a, long long n){
  long long r=1,i;
  for(i=0;i<n;i++){r*=a;}
  return r;
}

typedef struct{
  long long fi;
  long long se;
}data;

data d[262144];

void marge(long long x,long long y){
  if(x==y){return;}
  long long nf,ns;
  if(d[x].fi > d[y].fi){
    nf=d[x].fi;
    if(d[x].se > d[y].fi){
      ns=d[x].se;
    }
    else{
      ns=d[y].fi;
    }
  }
  else{
    nf=d[y].fi;
    if(d[x].fi > d[y].se){
      ns=d[x].fi;
    }
    else{
      ns=d[y].se;
    }
  }
  d[x].fi=nf;
  d[x].se=ns;
}

int main(){
  long long r=0,i,j,n,x,a[262144];
  scanf("%lld",&n);
  x=pow(2,n);
  for(i=0;i<x;i++){scanf("%lld",&a[i]);
    d[x-i-1].fi=a[i];d[x-i-1].se=0;
  }
  for(i=0;i<n;i++){
    for(j=0;j<x;j++){
      if((j>>i & 1) == 0){
        marge(j,(j|(1<<i)));
      }
    }
  }
  for(i=x-2;i>=0;i--){
    r=max(r,d[i].fi+d[i].se);
    printf("%lld\n",r);
  }
  return 0;
}