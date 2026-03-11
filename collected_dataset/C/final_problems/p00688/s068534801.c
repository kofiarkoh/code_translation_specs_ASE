#include <stdio.h>

int my_sqrt(int n) {
	int left,right,mid;
	if(n==0)return 0;
	if(n<0)return -1;
	left=1;right=n;
	while(left<=right) {
		mid=(left+right)/2;
		if(mid*mid==n)return mid;
		else if(mid*mid<n)left=mid+1;
		else right=mid-1;
	}
	return -1;
}

int solve_houteisiki(int* a1,int* a2,int a,int b,int c) {
	int D=b*b-4*a*c;
	int D_sqrt;
	if(D<0)return 0;
	D_sqrt=my_sqrt(D);
	if(D_sqrt<0)return 0;
	*a1=b-D_sqrt;
	*a2=b+D_sqrt;
	return 1;
}

int get_gcd(int a,int b) {
	if(a<0)a=-a;
	if(b<0)b=-b;
	if(a<b)return get_gcd(b,a);
	if(b==0)return a;
	return get_gcd(b,a%b);
}

int main(void) {
	int a,b,c;
	int p,q,r,s;
	int gcd;
	int t;
	while(1) {
		scanf("%d%d%d",&a,&b,&c);
		if(a==0 && b==0 && c==0)break;
		p=r=a*2;
		if(!solve_houteisiki(&q,&s,a,b,c)) {
			puts("Impossible");
		} else {
			gcd=get_gcd(p,q);
			p/=gcd;q/=gcd;
			gcd=get_gcd(r,s);
			r/=gcd;s/=gcd;
			if(p<r || (p==r && q<s)) {
				t=p;p=r;r=t;
				t=q;q=s;s=t;
			}
			if(p*r!=a)puts("Impossible");
			else printf("%d %d %d %d\n",p,q,r,s);
		}
	}
	return 0;
}