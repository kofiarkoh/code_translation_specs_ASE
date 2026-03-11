#include <stdio.h>

long int findprime(long int n){
 long int i;
  for(i=2;i<=n;i++){
    if(n%i==0){
      break;
    }
  }
  return i;
}

int primeindex(long int n,long int p){
  int i;
  for(i=1;;i++){
    if(n%p!=0){
      break;
    }
    n=n/p;
  }
  return i-1;
}

int numofdiv(int e){
  int i,s=0;
  for(i=1;;i++){
    s=s+i;
    if(e<s){
      break;
    }
  }
  return i-1;
}

long int expo(long int p, int e){
  int i;
  long int y=1;
  for(i=0;i<e;i++){
    y=y*p;
  }
  return y;
}

int main(){
  long int n;
  scanf("%ld",&n);

  int i,s=0,e,k[48],j;
  long int p;
  if(n>1){
    for(i=0;n>1;i++){
      p=findprime(n);
      e=primeindex(n,p);
      k[i]=numofdiv(e);
      n=n/expo(p,e);
    }
    for(j=0;j<=i;j++){
      s=s+k[j];
    }
  }

  printf("%d\n",s);
  return 0;
}