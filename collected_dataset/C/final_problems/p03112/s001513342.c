#include <stdio.h>
#include <stdlib.h>
#define LIM 100000
#define INF 1e18;

int binary_seaach(long long *array,int key,int min,int max){
	if(min>=max)return min;
	int mid = min + (max-min)/2;
	if(key<array[mid])return binary_seaach(array,key,min,mid);
	else if(array[mid]<key)return binary_seaach(array,key,mid+1,max);
	return mid;
}


int main(void){
	int A,B,Q,i;
	scanf("%d%d%d",&A,&B,&Q);
	long long s[LIM+2],t[LIM+2],ans,x,r;
	for(i=1;i<=A;i++)scanf("%lld",&s[i]);
	for(i=1;i<=B;i++)scanf("%lld",&t[i]);
	s[0] = t[0] = -INF;
	s[A+1] = t[B+1] = INF;
	for(i=0;i<Q;i++){
		scanf("%lld",&x);
		ans=INF;
		
		int a,b,c,d,j,k;
		b = binary_seaach(s,x,0,A+1);
		d = binary_seaach(t,x,0,B+1);
		a = b-1;
		c = d-1;
		
		for(j=a;j<=b;j++){
			for(k=c;k<=d;k++){
				r = llabs(s[j]-x)+llabs(t[k]-s[j]);
				ans = r<ans?r:ans;
				r = llabs(t[k]-x)+llabs(s[j]-t[k]);
				ans = r<ans?r:ans;
			}
		}
		
		
		printf("%lld\n",ans);
	}
	return 0;
}