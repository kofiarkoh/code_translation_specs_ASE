#include<stdio.h>
int N,B[100010]={0};
void bset(int n){for(N=1;N<n;N+=2);}
void badd(int i,int n){for(;i<=N;i+=i&-i)B[i]+=n;}
int  bsum(int i){
  int s=0;
  for(;i;i-=i&-i)s+=B[i];
  return s;
}		   
int main(){
  int n,m,a,b,c;
  scanf("%d %d",&n,&m);
  bset(n);
  while(m--){
    scanf("%d %d %d",&a,&b,&c);
    a?printf("%d\n",bsum(c)-bsum(b-1)):badd(b,c);
  }    
  return 0;
}