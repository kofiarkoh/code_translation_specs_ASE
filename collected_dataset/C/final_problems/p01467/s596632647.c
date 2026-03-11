#include<stdio.h>
#include<string.h>
int main(){
	int i,j,k,n,m,x;
	long long aa,bb;
	int a[15],b[15],c[15],f[15];
	char s[15],t[15],u;
	for(i=0;i<15;i++)b[i]=0,f[i]=0;
	scanf("%lld%lld%d",&aa,&bb,&n);
	x=0;while(aa>0)a[x]=aa%10,aa=aa/10,x++;
	m=x;
	x=0;while(bb>0)b[x]=bb%10,bb=bb/10,x++;
	x=0;
	for(i=0;i<m;i++){
		if(a[i]-x>=b[i])x=0;
		else{
			x=1;
			f[i+1]=1;
		}
	}
	aa=bb=0;
	for(i=m-1;i>=0;i--){
		if(n>0 && f[i]==1 && (f[i+1]==0 || a[i]-b[i]>=1 || (a[i]-b[i]<0) ) )n--,a[i]++;
		aa=aa*10;aa+=a[i];
		bb=bb*10;bb+=b[i];
	}
	//printf("%lld %lld\n",aa,bb);
	printf("%lld\n",aa-bb);
	return 0;
}