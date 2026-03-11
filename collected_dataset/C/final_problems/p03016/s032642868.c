#include<stdio.h>
#include<string.h>
#define MAX 100000
#define MOD 1000000007
#define df 0
int  dc=100000000;
typedef long long int li;

li m,l,a,d;


int keta(li n){
  int i=0;
  while(n){
    n=n/10;
    i++;
  }
  return i;
}
li pow_10(int r){
  int i;
  li s=1;
  for(i=0;i<r;i++)s*=10;
  return s;
}
li last(int r){
  li s=pow_10(r)-1;
  li k=(s-a)/d;
  if(df)printf("(10^r-1=%lld, last=%lld)\n",s,k);
  return k<l?k:l-1;
}

typedef struct {
  li a;
  li b;
  li c;
  li d;
  li e;
  li f;
} mat;

mat mul_mat(mat A,mat B){
  mat C;
  C.a=A.a*B.a%m;
  C.b=A.b*B.a%m+A.c*B.b%m;
  C.c=A.c*B.c%m;
  C.d=A.d*B.a%m+A.e*B.b%m+A.f*B.d%m;
  C.e=A.e*B.c%m+A.f*B.e%m;
  C.f=A.f*B.f%m;
  C.a%=m;
  C.b%=m;
  C.c%=m;
  C.d%=m;
  C.e%=m;
  C.f%=m;
  return C;
}

mat pow_mat(mat A,int n){
  dc-=df; if(dc<0){printf("err B");return A;}
  mat s=(mat){1,0,1,0,0,1},t=A;
  while(n){
    if(n&1) s=mul_mat(s,t);
    t=mul_mat(t,t);
    n/=2;
  }
  return s;
}

typedef struct{
  li a;
  li b;
  li c;
} vec;

vec mul_vec(vec v,mat A){
  vec u;
  u.a=v.a*A.a%m+v.b*A.b%m+v.c*A.d%m;
  u.b=v.b*A.c%m+v.c*A.e%m;
  u.c=v.c*A.f%m;
  u.a%=m;
  u.b%=m;
  u.c%=m;
  return u;
}

void print_mat(mat M){
  printf("%10lld %10lld %10lld\n",M.a,0,0);
  printf("%10lld %10lld %10lld\n",M.b,M.c,0);
  printf("%10lld %10lld %10lld\n\n",M.d,M.e,M.f);
}

void print_vec(vec v){
  printf("v:[%lld %lld %lld]\n",v.a,v.b,v.c);
}


int main(){
  scanf("%lld%lld%lld%lld",&l,&a,&d,&m);
  
  int i;
  li b,r,p=0,q;
  vec v={0,a%m,1};


  r=keta(a);
  q=-1;
  while(q!=l-1){
    dc-=df; if(dc<0){printf("err A");break;}
    q=last(r);
    if(df){
      printf("[%dketa; %lld ~ %lld]\n",r,p,q);
      print_vec(v);
      printf(">>>");
    }
    mat M={pow_10(r)%m,1,1,0,d%m,1};
    M=pow_mat(M,q-p+1);
    v=mul_vec(v,M);
    
    if(df){
      print_vec(v);
      printf("M=M(10^%d)^%d\n",r,q-p+1);
      print_mat(M);
    }
    p=q+1;
    r=keta(a+p*d);
  }
  

  
  printf("%lld",v.a);

}
