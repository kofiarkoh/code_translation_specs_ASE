#include<stdio.h>
#include<stdlib.h>
//絶対値の大きい順にソート
long long c[200002];
void merge(int i,int j,long long *s){
  if(i<j){
    merge(i,(i+j)/2,s);
    merge((i+j)/2+1,j,s);
    int a=i,b=(i+j)/2+1;
    int d;
    for(d=i;d<=j;d++){
      if((i+j)/2<a){
        c[d]=s[b];
        b++;
      }
      else if(j<b){
        c[d]=s[a];
        a++;
      }
      else if(llabs(s[a])>llabs(s[b])){
        c[d]=s[a];
        a++;
      }
      else{
        c[d]=s[b];
        b++;
      }
    }
    for(d=i;d<=j;d++)
      s[d]=c[d];
  }
}

int main(){
  int n,k,i;
  scanf("%d%d",&n,&k);
  long long plus[n+2],minus[n+2],seki=1;
  int plusnum=0,minusnum=0;
  for(i=0;i<n;i++){
    int num;
    scanf("%d",&num);
    if(num<=0){
      minus[minusnum]=num;
      minusnum++;
    }
    else{
      plus[plusnum]=num;
      plusnum++;
    }
  }
  merge(0,minusnum-1,minus);
  merge(0,plusnum-1,plus);
  minus[minusnum]=0;
  minus[minusnum+1]=0;
  plus[plusnum]=0;
  plus[plusnum+1]=0;
  if(plusnum==0 && k%2==1){
    for(i=0;i<k;i++){
      seki*=minus[minusnum-i-1];
      seki%=1000000007;
      if(seki<0)
        seki+=1000000007;
    }
  }
  else if(k%2==1){
    int j=0,l=1;
    long long pl=plus[1]*plus[2];
    long long mi=minus[0]*minus[1];
    seki=plus[0];
    for(i=0;i<k/2;i++){
      if(pl<mi){
        mi%=1000000007;
        seki*=mi;
        j++;
        mi=minus[2*j]*minus[2*j+1];
      }
      else{
        pl%=1000000007;
        seki*=pl;
        l++;
        pl=plus[2*l]*plus[2*l-1];
      }
      seki%=1000000007;
    }
  }
  else{
    int j=0,l=0;
    long long pl=plus[0]*plus[1];
    long long mi=minus[0]*minus[1];
    for(i=0;i<k/2;i++){
      if(pl<mi){
        mi%=1000000007;
        seki*=mi;
        j++;
        mi=minus[2*j]*minus[2*j+1];
      }
      else{
        pl%=1000000007;
        seki*=pl;
        l++;
        pl=plus[2*l]*plus[2*l+1];
      }
      seki%=1000000007;
    }
  }
  printf("%lld\n",seki);
  return 0;
}
