#include<stdio.h>
#include<stdlib.h>

struct v{
	long long a;
	long long b;
	int na;
};

int compare_int(const void *a,const void *b){
	if(*(long long*)a>*(long long*)b)return 1;
	return -1;
}
int compare_intt(const void *a,const void *b){
	struct v A=*(struct v*)a,B=*(struct v*)b;
	if(A.b>B.b)return 1;
	else if(A.b==B.b){
		if(A.a>B.a)return 1;
	}return -1;
}

int main(){
	int n,i;
	struct v b[100000];
	long long acp[100000];
	scanf("%d",&n);
	for(i=0;i<n;i++)scanf("%lld",&b[i].a),acp[i]=b[i].a;
	for(i=0;i<n;i++)scanf("%lld",&b[i].b);
	qsort(acp,n,sizeof(long long),compare_int);
	for(i=0;i<n;i++){
		long long am=b[i].a;
		int r=n,l=-1;
		while(r-l>1){
			int mid=(r+l)/2;
			if(acp[mid]>=am)r=mid;
			else l=mid;
		}
		if(acp[r]==b[i].a)b[i].na=r;
		else b[i].na=l;
	}
	qsort(b,n,sizeof(b[0]),compare_intt);
	for(i=0;i<n;i++){
		if(acp[i]>b[i].b){
			puts("No");
			return 0;
		}
	}
	if(n==2){
		if(b[0].a>b[0].b){
			puts("No");
		}else puts("Yes");
		return 0;
	}
	for(i=0;i<n;i++){
		if(acp[i]==b[i].a){
			puts("Yes");
			return 0;
		}
	}
	int c=0,co=0;
	while(1){
		c=b[c].na;
		if(c==0)break;
		co++;
	}
//	printf("%d\n",co);
	if(co<n-1)puts("Yes");
	else puts("No");
	return 0;
}