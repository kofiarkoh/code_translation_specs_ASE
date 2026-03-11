#include<stdio.h>
#include<stdlib.h>
#define df 0
#define d_flag 0

int  d_count=10000000;
#define MOD 1000000007

typedef long int li;
li waru(li a,li b);
li kake(li a,li b);
li inv(li a);


int main(){
  int n,k;
  scanf("%d%d",&n,&k);
  li R=n-k,B=k,i;
  li bb,rr,ans;
  bb=1;
  rr=R+1;
  ans=rr;
  printf("%ld\n",ans);
  
  for(i=2;i<=k;i++){
    bb=waru(kake(bb,B-i+1),i-1);
    rr=waru(kake(rr,R-i+2),i);
    ans=kake(bb,rr);
    printf("%ld\n",ans);
  }
    
  

  return 0;
}
li inv(li a){
  li x0,y0,x1,y1,z0,z1,q,x2,y2,z2;
  x0=0;y0=1;z0=MOD;
  x1=1;y1=0;z1=a;
  if(d_flag)printf("\n");
  if(d_flag)printf("%10ld %10ld %10ld\n",x0,y0,z0);
  if(d_flag)printf("%10ld %10ld %10ld\n",x1,y1,z1);
  while(z1!=1){
    d_count-=d_flag; if(d_count<0){printf("err:inv\n");break;}
    q=z0/z1;
    x2=x0-x1*q;
    y2=y0-y1*q;
    z2=z0-z1*q;
    x0=x1;y0=y1;z0=z1;
    x1=x2;y1=y2;z1=z2;
  if(d_flag)printf("%10ld %10ld %10ld\n",x1,y1,z1);
  }
  return (MOD+x1)%MOD;
}
li waru(li a,li b){
  if(d_flag)printf("calc:%ld/%ld",a,b);
  if(a%b==0){
    if(d_flag) printf("->%ld\n",a/b);
    return a/b;
  }
  else{
    li c=inv(b);
    c=kake(a,c);
    if(d_flag) printf("->%ld\n",c);
    return(c);
  }
}

li kake(li a,li b){
  //  if(d_flag)printf("%ld*%ld\n",a%MOD,b%MOD);
  a%=MOD;
  b%=MOD;
  return a*b%MOD;
}

/// confirm df==0 ///
