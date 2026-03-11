#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#define ll long long
#define MOD 1000000007LL
#define len 12
int scan(int*a,int n){int*p;char*ptr_c,*buf;buf=malloc(n*len);p=a;ptr_c=buf;fgets(buf,n*len,stdin);while(n--){*p++=atoi(ptr_c);ptr_c=strchr(ptr_c+1,32)+1;}}

int main(){
	ll a,b;
	int *x,*y,*ptr,n,m,i;

	scanf("%d%d\n",&n,&m);
	x=(int*)malloc(n*sizeof(int));
	y=(int*)malloc(m*sizeof(int));
	scan(x,n);
	scan(y,m); 

	for(i=1,a^=a,ptr=x,++n;i<n;++i)
		a+=*ptr++*((ll)i+i-n)%MOD;
		
	for(i=1,b^=b,ptr=y,++m;i<m;++i)
		b+=*ptr++*((ll)i+i-m)%MOD;
	
	printf("%lld\n",(a%MOD)*(b%MOD)%MOD);
	
	free(x);
	free(y);
	return 0;
}
