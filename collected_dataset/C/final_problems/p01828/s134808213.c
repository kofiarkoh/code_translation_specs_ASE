#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <ctype.h>
#include <limits.h>	
#define inf INT_MAX
#define INF 9223372036854775807
#define sq(n) ((n)*(n))
#define rep(i,n) for(i=0;i<n;i++)
#define rev(i,n) for(i=n-1;i>=0;i--)

int in(void){
	int i;scanf("%d",&i);
	return i;
}
long long llin(void){
	long long i;scanf("%lld",&i);
	return i;
}
double din(void){
	double i;scanf("%lf",&i);
	return i;
}
void chin(char s[]){
	scanf("%s",s);
}
void print(int a){
	printf("%d\n",a);
}
void llprint(long long a){
	printf("%lld\n",a);
}
void dprint(double a){
	printf("%.10f\n",a);
}
void print2(int a,int b){
	printf("%d %d\n",a,b);
}
long long max(long long a,long long b){
	return a>b?a:b;
}
long long min(long long a,long long b){
	return a<b?a:b;
}
double dmax(double a,double b){
	return a>b?a:b;
}
int cmp(const void *a,const void *b){
	return *(int *)a-*(int *)b;
}

int main(void){
	char s[1001],t[1001],w[1001];
	int i,j,f=0;
	chin(s);
	chin(t);
	for(i=0,j=0;i<strlen(s);i+=2,j++){
		w[j]=s[i];
	}
	w[j]=0;
	for(i=0,j=0;i<strlen(t)&&j<strlen(w);i++){
		if(t[i]==w[j]){
			j++;
		}
	}
	if(j==strlen(w)){
		f=1;
	}
	for(i=1,j=0;i<strlen(s);i+=2,j++){
		w[j]=s[i];
	}
	w[j]=0;
	for(i=0,j=0;i<strlen(t)&&j<strlen(w);i++){
		if(t[i]==w[j]){
			j++;
		}
	}
	if(j==strlen(w)){
		f=1;
	}
	puts(f?"Yes":"No");
	return 0;
}