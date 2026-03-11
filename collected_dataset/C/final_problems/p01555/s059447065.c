#include<stdio.h>
char s[10010];
int r=0;
void f(long long a){
	char d[20],i;
	for(i=0;a;a/=10)d[i++]=a%10+'0';
	while(i--)s[r++]=d[i];
}
int main(){
	long long n,a=1,b,i,j;
	char c[][5]={"Fizz","Buzz"};
	scanf("%lld",&n);//printf("%lld\n",n);fflush(stdout);
	for(i=2;n>=6*a*8*(i+4)-5;i++){
		n-=6*a*8*(i+4)-5;
		a*=10;
	}//printf("%lld\n",n);
	if(8*(i+4)-5<n)n+=5;
	b=n/(8*(i+4));
	n%=8*(i+4);//printf("%lld %lld %lld\n",a,b,n);fflush(stdout);
	for(i=1;i<=30;i++){
		if(i%3==0){
			for(j=0;j<4;j++)s[r++]=c[0][j];
		}
		if(i%5==0){
			for(j=0;j<4;j++)s[r++]=c[1][j];
		}
		if(i%3&&i%5)f(a*10+(b-1)*15+i+5);
	}
	s[n+19]=0;
	printf("%s\n",s+n-1);
	return 0;
}
