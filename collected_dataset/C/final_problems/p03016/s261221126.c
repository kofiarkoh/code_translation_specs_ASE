#include<stdio.h>

long long mod;

void matpow(long long a[3][3],long long k){
  long long i,j,r[3][3],mem[3][3],t;
  for(i=0;i<3;i++){
    for(j=0;j<3;j++){
      if(i==j){r[i][j]=1;}
      else{r[i][j]=0;}
    }
  }
  while(k>0){
    if(k%2==1){
      for(i=0;i<3;i++){
        for(j=0;j<3;j++){
          mem[i][j]=0;
          for(t=0;t<3;t++){
            mem[i][j]+=r[i][t]*a[t][j];
            mem[i][j]%=mod;
          }
        }
      }
      for(i=0;i<3;i++){
        for(j=0;j<3;j++){
          r[i][j]=mem[i][j];
        }
      }
    }
    k/=2;
    for(i=0;i<3;i++){
      for(j=0;j<3;j++){
        mem[i][j]=0;
        for(t=0;t<3;t++){
          mem[i][j]+=a[i][t]*a[t][j];
          mem[i][j]%=mod;
        }
      }
    }
    for(i=0;i<3;i++){
      for(j=0;j<3;j++){
        a[i][j]=mem[i][j];
      }
    }
  }
  for(i=0;i<3;i++){
    for(j=0;j<3;j++){
      a[i][j]=r[i][j];
    }
  }
}

long long power(long long a, long long b){
	long long x = 1, y = a;
	while (b > 0){
		if(b%2){
			x = (x*y)%mod;
		}
		y = (y*y)%mod;
		b /= 2;
	}
	return x%mod;
}

int main(){
  long long l,a,b,k=10,r=0;
  long long st,fi,te;
  long long mat[3][3];
  scanf("%lld%lld%lld%lld",&l,&a,&b,&mod);
  while(l>0){
    if(a>=k){k*=10;continue;}
    st=0;fi=l-1;
    while(st<=fi){
      te=(st+fi)/2;
      if((a+b*te)>=k){fi=te-1;}
      else{st=te+1;}
    }
    if(fi==-1){k*=10;continue;}
    mat[0][0]=k%mod;
    mat[0][1]=(mod-(k%mod))%mod;
    mat[0][2]=0;
    mat[1][0]=0;mat[1][1]=k%mod;mat[1][2]=0;
    mat[2][0]=1;mat[2][1]=0;mat[2][2]=1;
    matpow(mat,fi+1);
    r*=power(k%mod,fi+1);r%=mod;
    r+=(mat[2][0]*((a+fi*b)%mod));r%=mod;
    r+=(mat[2][1]*(b%mod));r%=mod;
    l-=(fi+1);
    a=a+b*st;
    k*=10;
  }
  printf("%lld\n",r);
}