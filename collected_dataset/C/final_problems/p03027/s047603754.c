#include<stdio.h>
#include<stdlib.h>
#define MOD 1000003
#define MAX 100000

#define d_flag 0
int  d_count=10000000;

typedef long int li;
li waru(li a,li b);
li kake(li a,li b);
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
li Dpow(li x,li n){
  if(n==0)return 1;
  if(n%2)return(kake(x,Dpow(x,n-1)));
  if(n%2)return(kake(Dpow(x,n/2),Dpow(x,n/2)));
}
void solve(li x,li d,li n){
  li ans;
  if(d==0)ans=Dpow(x,n);
  else if(n>1000002)ans=0;
  else if(n> 500001){
    int i;
    ans=-1;
    n=MOD-n-1;
    for(i=0;i<n;i++){
      x=(x-d)%MOD;
      ans=kake(ans,x);
    }
  }else{
    int i;
    ans=1;
    for(i=0;i<n;i++){
      ans=kake(ans,x);
      x=(x+d)%MOD;
    }
  }
  printf("%ld\n",ans);
}
int main(){
  if(d_flag)printf("-7\%3=%d",-7%3);
  int Q;
  int x[MAX],d[MAX],n[MAX],i;
  scanf("%d",&Q);
  for(i=0;i<Q;i++){
    scanf("%d%d%d",&x[i],&d[i],&n[i]);
  }
  for(i=0;i<Q;i++){
    solve(x[i],d[i],n[i]);
  }

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
  if(d_flag)printf("%ld*%ld\n",a%MOD,b%MOD);
  return(((a%MOD)*(b%MOD))%MOD);
}
